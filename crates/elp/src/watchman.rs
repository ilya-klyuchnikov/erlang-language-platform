/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::sync::Arc;

use anyhow::Result;
use elp_ide::elp_ide_db::elp_base_db::AbsPathBuf;
use elp_ide::elp_ide_db::elp_base_db::SourceDatabase;
use elp_ide::elp_ide_db::elp_base_db::SourceDatabaseExt;
use elp_ide::elp_ide_db::elp_base_db::SourceRoot;
use elp_ide::elp_ide_db::elp_base_db::SourceRootId;
use elp_ide::elp_ide_db::elp_base_db::VfsPath;
use paths::Utf8PathBuf;
use serde::Deserialize;

use crate::build::types::LoadResult;
use crate::document::Document;

pub struct Watchman {
    watch: PathBuf,
    clock: String,
    expression: serde_json::Value,
}

#[derive(Deserialize)]
struct WatchProjectResponse {
    watch: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
struct WatchmanQueryResult {
    #[serde(default)]
    is_fresh_instance: bool,
    clock: String,
    #[serde(default)]
    files: Vec<WatchmanFile>,
}

#[derive(Debug, Clone, Deserialize)]
struct WatchmanFile {
    name: String,
    exists: bool,
    #[serde(default)]
    new: bool,
}

impl Watchman {
    pub fn new(project: &Path) -> Result<Self> {
        let mut cmd = Command::new("watchman");
        cmd.arg("watch-project").arg(project.as_os_str());
        let output = cmd.output().map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                anyhow::Error::msg(
                    "`watchman` command not found. install it from \
                     https://facebook.github.io/watchman/ to use `elp shell`.",
                )
            } else {
                e.into()
            }
        })?;
        let resp: WatchProjectResponse = serde_json::from_slice(&output.stdout).map_err(|_| {
            anyhow::Error::msg(
                "Could not find project. Are you in an Erlang project directory, \
                     or is one specified using --project?",
            )
        })?;
        let clock = Self::get_clock_for(&resp.watch)?;
        let expression = build_watchman_expression(&[]);
        Ok(Watchman {
            watch: resp.watch,
            clock,
            expression,
        })
    }

    pub fn set_project_dirs(&mut self, loaded: &LoadResult) {
        let project_dirs = collect_project_dirs(loaded, &self.watch);
        self.expression = build_watchman_expression(&project_dirs);
        // Best-effort clock refresh; keep old clock on failure
        if let Ok(clock) = Self::get_clock_for(&self.watch) {
            self.clock = clock;
        }
    }

    pub fn poll_and_apply_changes(&mut self, loaded: &mut LoadResult) -> Result<UpdateResult> {
        let result = self.query()?;

        if result.is_fresh_instance {
            return Ok(UpdateResult::NeedsFullReload {
                reason: "Watchman clock invalidated (large change detected), reloading project...",
            });
        }

        if result.files.is_empty() {
            self.clock = result.clock;
            return Ok(UpdateResult::Updated);
        }

        if result.files.iter().any(|f| is_elp_config_file(&f.name)) {
            return Ok(UpdateResult::NeedsRestart {
                reason: "ELP config change detected, restart required",
            });
        }

        if result.files.iter().any(|f| is_config_file(&f.name)) {
            return Ok(UpdateResult::NeedsFullReload {
                reason: "Project change detected, reloading project",
            });
        }

        if result
            .files
            .iter()
            .any(|f| is_suite_file(&f.name) && (f.new || !f.exists))
        {
            return Ok(UpdateResult::NeedsFullReload {
                reason: "Suite file change detected, reloading project",
            });
        }

        let vfs = &mut loaded.vfs;
        for file in &result.files {
            let path = self.watch.join(&file.name);
            let vfs_path = VfsPath::from(AbsPathBuf::assert(
                Utf8PathBuf::from_path_buf(path.clone()).expect("UTF8 conversion failed"),
            ));
            if !file.exists {
                vfs.set_file_contents(vfs_path, None);
            } else {
                match fs::read(&path) {
                    Ok(contents) => {
                        vfs.set_file_contents(vfs_path, Some(contents));
                    }
                    Err(err) => {
                        log::warn!("Cannot read file {path:?}: {err}, treating as deleted");
                        vfs.set_file_contents(vfs_path, None);
                    }
                }
            }
        }
        process_changes_to_vfs_store(loaded);

        self.clock = result.clock;
        Ok(UpdateResult::Updated)
    }

    fn get_clock_for(watch: &Path) -> Result<String> {
        let mut cmd = Command::new("watchman");
        cmd.arg("clock");
        cmd.arg(watch.as_os_str());
        let result: serde_json::Value = serde_json::from_slice(&cmd.output()?.stdout)?;
        result["clock"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("Missing clock in watchman response"))
    }

    fn query(&self) -> Result<WatchmanQueryResult> {
        let query = serde_json::json!([
            "query",
            self.watch.to_string_lossy(),
            {
                "since": self.clock,
                "expression": self.expression,
                "fields": ["name", "exists", "new"]
            }
        ]);
        let mut cmd = Command::new("watchman");
        cmd.arg("-j")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = cmd.spawn()?;
        if let Some(stdin) = child.stdin.take() {
            serde_json::to_writer(stdin, &query)?;
        }
        let output = child.wait_with_output()?;
        Ok(serde_json::from_slice(&output.stdout)?)
    }
}

pub enum UpdateResult {
    Updated,
    NeedsFullReload { reason: &'static str },
    NeedsRestart { reason: &'static str },
}

fn collect_project_dirs(loaded: &LoadResult, watch_root: &Path) -> Vec<PathBuf> {
    let prefix = format!("{}/", watch_root.display());
    loaded
        .project
        .non_otp_apps()
        .flat_map(|app| app.all_dirs_to_watch())
        .filter_map(|dir| {
            let dir_str = dir.to_string();
            dir_str.strip_prefix(&prefix).map(PathBuf::from)
        })
        .collect()
}

fn build_watchman_expression(project_dirs: &[PathBuf]) -> serde_json::Value {
    let config_files = serde_json::json!([
        "anyof",
        ["match", "BUCK", "basename"],
        ["match", "TARGETS", "basename"],
        ["match", "TARGETS.v2", "basename"],
        ["match", "rebar.config", "basename"],
        ["match", "rebar.config.script", "basename"],
        ["match", ".elp.toml", "basename"],
        ["match", ".elp_lint.toml", "basename"]
    ]);

    let source_suffix = serde_json::json!(["anyof", ["suffix", "erl"], ["suffix", "hrl"]]);

    let source_files = if project_dirs.is_empty() || project_dirs.len() > 100 {
        source_suffix
    } else {
        let mut dir_parts: Vec<serde_json::Value> = vec![serde_json::json!("anyof")];
        for dir in project_dirs {
            dir_parts.push(serde_json::json!(["dirname", dir.to_string_lossy()]));
        }
        let dir_filter = serde_json::Value::Array(dir_parts);
        serde_json::json!(["allof", source_suffix, dir_filter])
    };

    serde_json::json!([
        "allof",
        ["type", "f"],
        ["anyof", source_files, config_files]
    ])
}

fn is_elp_config_file(name: &str) -> bool {
    let basename = Path::new(name)
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("");
    matches!(basename, ".elp.toml" | ".elp_lint.toml")
}

fn is_config_file(name: &str) -> bool {
    let basename = Path::new(name)
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("");
    matches!(
        basename,
        "BUCK" | "TARGETS" | "TARGETS.v2" | "rebar.config" | "rebar.config.script"
    )
}

fn is_suite_file(name: &str) -> bool {
    name.ends_with("_SUITE.erl")
}

pub(crate) fn process_changes_to_vfs_store(loaded: &mut LoadResult) -> bool {
    let changed_files = loaded.vfs.take_changes();

    if changed_files.is_empty() {
        return false;
    }

    let raw_database = loaded.analysis_host.raw_database_mut();

    for (_, file) in &changed_files {
        let file_exists = loaded.vfs.exists(file.file_id);
        if file.change != vfs::Change::Delete && file_exists {
            if let vfs::Change::Create(v, _) | vfs::Change::Modify(v, _) = &file.change {
                let document = Document::from_bytes(v);
                let (text, line_ending) = document.vfs_to_salsa();
                raw_database.set_file_text(file.file_id, Arc::from(text));
                loaded.line_ending_map.insert(file.file_id, line_ending);
            } else {
                raw_database.set_file_text(file.file_id, Arc::from(""));
            };
        }
    }

    if changed_files
        .into_values()
        .any(|file| file.is_created_or_deleted())
    {
        let sets = loaded.file_set_config.partition(&loaded.vfs);
        for (idx, set) in sets.into_iter().enumerate() {
            let root_id = SourceRootId(idx as u32);
            for file_id in set.iter() {
                raw_database.set_file_source_root(file_id, root_id);
            }
            let root = SourceRoot::new(set);
            raw_database.set_source_root(root_id, Arc::new(root));
        }
    }

    true
}
