/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

use std::mem;

use fxhash::FxHashMap;
use itertools::Itertools;

use super::types::CommentFact;
use super::types::Declaration;
use super::types::Fact;
use super::types::FileDeclaration;
use super::types::FileFact;
use super::types::FileLinesFact;
use super::types::FunctionDeclarationFact;
use super::types::GleanFileId;
use super::types::Key;
use super::types::Location;
use super::types::MFA;
use super::types::ModuleFact;
use super::types::Schema2CallbackDecl;
use super::types::Schema2CallbackDef;
use super::types::Schema2CommentFact;
use super::types::Schema2DeclLocation;
use super::types::Schema2Declaration;
use super::types::Schema2FileDeclarations;
use super::types::Schema2FileIncludes;
use super::types::Schema2Fqn;
use super::types::Schema2FuncDecl;
use super::types::Schema2FuncDef;
use super::types::Schema2HeaderDecl;
use super::types::Schema2MacroDecl;
use super::types::Schema2MacroDef;
use super::types::Schema2ModuleDecl;
use super::types::Schema2ModuleDef;
use super::types::Schema2RecordDecl;
use super::types::Schema2RecordDef;
use super::types::Schema2RecordFieldDecl;
use super::types::Schema2TypeDecl;
use super::types::Schema2TypeDef;
use super::types::Schema2VarDecl;
use super::types::Schema2VarLocation;
use super::types::Schema2VarXRef;
use super::types::Schema2VarXRefsByFile;
use super::types::Schema2XRef;
use super::types::Schema2XRefsByFile;
use super::types::XRefFact;
use super::types::XRefFactVal;
use super::types::XRefFile;
use super::types::XRefTarget;

const REC_ARITY: u32 = 99;
const HEADER_ARITY: u32 = 100;

#[derive(Debug, Default, Clone)]
pub(crate) struct IndexedFacts {
    pub(crate) file_facts: Vec<FileFact>,
    pub(crate) file_line_facts: Vec<FileLinesFact>,
    pub(crate) module_facts: Vec<ModuleFact>,
    pub(crate) file_declarations: Vec<FileDeclaration>,
    pub(crate) xrefs: Vec<XRefFile>,
}

impl IndexedFacts {
    pub(crate) fn new(
        file_fact: FileFact,
        line_fact: FileLinesFact,
        decl: FileDeclaration,
        xref: XRefFile,
        module_fact: Option<ModuleFact>,
    ) -> Self {
        let mut facts = Self::default();
        facts.file_facts.push(file_fact);
        facts.file_line_facts.push(line_fact);
        facts.file_declarations.push(decl);
        facts.xrefs.push(xref);
        if let Some(module_fact) = module_fact {
            facts.module_facts.push(module_fact);
        }
        facts
    }

    pub(crate) fn add(
        &mut self,
        file_fact: FileFact,
        line_fact: FileLinesFact,
        decl: FileDeclaration,
        xref: XRefFile,
        module_fact: Option<ModuleFact>,
    ) {
        self.file_facts.push(file_fact);
        self.file_line_facts.push(line_fact);
        self.file_declarations.push(decl);
        self.xrefs.push(xref);
        if let Some(module_fact) = module_fact {
            self.module_facts.push(module_fact);
        }
    }

    fn declaration_to_fact(
        decl: Declaration,
        file_id: GleanFileId,
        module: String,
    ) -> Option<FunctionDeclarationFact> {
        let fact = match decl {
            Declaration::FunctionDeclaration(d) => {
                let fqn = MFA {
                    module,
                    name: d.key.name,
                    arity: d.key.arity,
                    file_id: file_id.clone(),
                };
                FunctionDeclarationFact {
                    file_id,
                    fqn,
                    span: d.key.span,
                }
            }
            Declaration::MacroDeclaration(d) => {
                let fqn = MFA {
                    module,
                    name: d.key.name,
                    arity: d.key.arity.unwrap_or(0),
                    file_id: file_id.clone(),
                };
                FunctionDeclarationFact {
                    file_id,
                    fqn,
                    span: d.key.span,
                }
            }
            Declaration::TypeDeclaration(d) => {
                let fqn = MFA {
                    module,
                    name: d.key.name,
                    arity: d.key.arity,
                    file_id: file_id.clone(),
                };
                FunctionDeclarationFact {
                    file_id,
                    fqn,
                    span: d.key.span,
                }
            }
            Declaration::RecordDeclaration(d) => {
                let fqn = MFA {
                    module,
                    name: d.key.name,
                    arity: REC_ARITY,
                    file_id: file_id.clone(),
                };
                FunctionDeclarationFact {
                    file_id,
                    fqn,
                    span: d.key.span,
                }
            }
            Declaration::HeaderDeclaration(d) => {
                let fqn = MFA {
                    module,
                    name: d.key.name,
                    arity: HEADER_ARITY,
                    file_id: file_id.clone(),
                };
                FunctionDeclarationFact {
                    file_id,
                    fqn,
                    span: d.key.span,
                }
            }
            Declaration::VarDeclaration(d) => {
                let fqn = MFA {
                    module,
                    name: d.key.name,
                    arity: d.key.span.start,
                    file_id: file_id.clone(),
                };
                FunctionDeclarationFact {
                    file_id,
                    fqn,
                    span: d.key.span,
                }
            }
            Declaration::DocDeclaration(_) => return None,
        };
        Some(fact)
    }

    pub(crate) fn into_glean_facts(
        mut self,
        modules: &FxHashMap<GleanFileId, String>,
    ) -> Vec<Fact> {
        let file_lines_fact = mem::take(&mut self.file_line_facts);
        let file_lines_fact = file_lines_fact.into_iter().map_into().collect();
        let declaration_fact = mem::take(&mut self.file_declarations);
        let mut declarations = vec![];
        let mut comments = vec![];
        for decl in declaration_fact {
            if let Some(module) = modules.get(&decl.file_id) {
                for d in decl.declarations {
                    let file_id = decl.file_id.clone();
                    if let Declaration::DocDeclaration(doc) = &d {
                        let declaration = doc.key.target.as_ref();
                        if let Some(target) = Self::declaration_to_fact(
                            declaration.clone(),
                            file_id.clone(),
                            module.clone(),
                        ) {
                            comments.push(
                                CommentFact {
                                    file_id,
                                    declaration: target.into(),
                                    span: doc.key.span.clone(),
                                    text: doc.key.text.clone(),
                                }
                                .into(),
                            );
                            continue;
                        }
                    }
                    if let Some(fact) = Self::declaration_to_fact(d, file_id, module.clone()) {
                        declarations.push(fact);
                    }
                }
            }
        }
        let declaration_fact = declarations.into_iter().map_into().collect();
        let xref_fact = mem::take(&mut self.xrefs);
        let mut xrefs = vec![];
        for fact in xref_fact {
            let file_id = fact.file_id;
            let mut facts = vec![];
            for xref in fact.xrefs {
                let source = xref.source;
                let file_id = xref.target.file_id();
                if let Some(module) = modules.get(file_id) {
                    let target = match xref.target {
                        XRefTarget::Function(x) => MFA {
                            module: module.clone(),
                            name: x.key.name,
                            arity: x.key.arity,
                            file_id: x.key.file_id,
                        },
                        XRefTarget::Macro(x) => MFA {
                            module: module.clone(),
                            name: x.key.name,
                            arity: x.key.arity.unwrap_or(0),
                            file_id: x.key.file_id,
                        },
                        XRefTarget::Header(x) => MFA {
                            module: module.clone(),
                            name: x.key.name,
                            arity: HEADER_ARITY,
                            file_id: x.key.file_id,
                        },
                        XRefTarget::Record(x) => MFA {
                            module: module.clone(),
                            name: x.key.name,
                            arity: REC_ARITY,
                            file_id: x.key.file_id,
                        },
                        XRefTarget::Type(x) => MFA {
                            module: module.clone(),
                            name: x.key.name,
                            arity: x.key.arity,
                            file_id: x.key.file_id,
                        },
                        XRefTarget::Var(x) => MFA {
                            module: module.clone(),
                            name: x.key.name,
                            arity: source.start,
                            file_id: x.key.file_id,
                        },
                    };
                    let val = XRefFactVal { source, target };
                    facts.push(val);
                }
            }
            xrefs.push(XRefFact {
                file_id,
                xrefs: facts,
            });
        }
        let xref_fact = xrefs.into_iter().map_into().collect();
        let module_facts = mem::take(&mut self.module_facts);
        let module_facts = module_facts.into_iter().map_into().collect();
        vec![
            Fact::File {
                facts: mem::take(&mut self.file_facts),
            },
            Fact::FileLine {
                facts: file_lines_fact,
            },
            Fact::FunctionDeclaration {
                facts: declaration_fact,
            },
            Fact::XRef { facts: xref_fact },
            Fact::DeclarationComment { facts: comments },
            Fact::Module {
                facts: module_facts,
            },
        ]
    }

    /// Convert internal facts to erlang.2 schema output (dual-write).
    pub(crate) fn into_schema2_facts(
        mut self,
        modules: &FxHashMap<GleanFileId, String>,
        apps: &FxHashMap<GleanFileId, String>,
    ) -> Vec<Fact> {
        let unknown = "unknown".to_string();

        let mut func_decls: Vec<Key<Schema2FuncDecl>> = vec![];
        let mut macro_decls: Vec<Key<Schema2MacroDecl>> = vec![];
        let mut record_decls: Vec<Key<Schema2RecordDecl>> = vec![];
        let mut type_decls: Vec<Key<Schema2TypeDecl>> = vec![];
        let mut header_decls: Vec<Key<Schema2HeaderDecl>> = vec![];
        let mut callback_decls: Vec<Key<Schema2CallbackDecl>> = vec![];
        let record_field_decls: Vec<Key<Schema2RecordFieldDecl>> = vec![]; // TODO: populate
        let mut var_decls: Vec<Key<Schema2VarDecl>> = vec![];
        let mut func_defs: Vec<Key<Schema2FuncDef>> = vec![];
        let macro_defs: Vec<Key<Schema2MacroDef>> = vec![]; // TODO: populate with definition_text
        let record_defs: Vec<Key<Schema2RecordDef>> = vec![]; // TODO: populate with definition_text
        let mut type_defs: Vec<Key<Schema2TypeDef>> = vec![];
        let mut callback_defs: Vec<Key<Schema2CallbackDef>> = vec![];
        let mut decl_locations: Vec<Key<Schema2DeclLocation>> = vec![];
        let mut var_locations: Vec<Key<Schema2VarLocation>> = vec![];
        let mut comments: Vec<Key<Schema2CommentFact>> = vec![];
        let mut file_decls_list: Vec<Key<Schema2FileDeclarations>> = vec![];

        let on_load_by_file: FxHashMap<GleanFileId, Vec<String>> = self
            .module_facts
            .iter()
            .filter(|mf| !mf.on_load_fns.is_empty())
            .map(|mf| (mf.file_id.clone(), mf.on_load_fns.clone()))
            .collect();
        let nif_by_file: FxHashMap<GleanFileId, Vec<(String, u32)>> = self
            .module_facts
            .iter()
            .filter(|mf| !mf.nif_fns.is_empty())
            .map(|mf| (mf.file_id.clone(), mf.nif_fns.clone()))
            .collect();

        let file_declarations = mem::take(&mut self.file_declarations);
        for file_decl in file_declarations {
            let module = modules.get(&file_decl.file_id).unwrap_or(&unknown);
            let app = apps.get(&file_decl.file_id).unwrap_or(&unknown);
            let module_on_load_fns = on_load_by_file.get(&file_decl.file_id);
            let module_nif_fns = nif_by_file.get(&file_decl.file_id);
            let mut file_schema2_decls: Vec<Schema2Declaration> = vec![];

            for d in file_decl.declarations {
                match d {
                    Declaration::FunctionDeclaration(ref f) => {
                        let decl = Schema2FuncDecl {
                            fqn: Schema2Fqn {
                                module: module.clone(),
                                name: f.key.name.clone(),
                                arity: f.key.arity,
                            },
                            app: app.clone(),
                        };
                        let s2decl = Schema2Declaration::Func(decl.clone().into());
                        decl_locations.push(
                            Schema2DeclLocation {
                                declaration: s2decl.clone(),
                                file_id: file_decl.file_id.clone(),
                                span: f.key.span.clone(),
                            }
                            .into(),
                        );
                        let is_on_load =
                            module_on_load_fns.is_some_and(|fns| fns.contains(&f.key.name));
                        func_defs.push(
                            Schema2FuncDef {
                                declaration: decl.clone().into(),
                                exported: f.key.exported,
                                deprecated: if f.key.deprecated {
                                    Some("true".to_string())
                                } else {
                                    None
                                },
                                is_on_load,
                                is_nif: module_nif_fns.is_some_and(|fns| {
                                    fns.iter().any(|(name, arity)| {
                                        name == &f.key.name && *arity == f.key.arity
                                    })
                                }),
                                spec_text: None,
                            }
                            .into(),
                        );
                        file_schema2_decls.push(s2decl);
                        func_decls.push(decl.into());
                    }
                    Declaration::MacroDeclaration(ref m) => {
                        let decl = Schema2MacroDecl {
                            name: m.key.name.clone(),
                            arity: m.key.arity,
                            module: module.clone(),
                            app: app.clone(),
                        };
                        let s2decl = Schema2Declaration::Macro(decl.clone().into());
                        decl_locations.push(
                            Schema2DeclLocation {
                                declaration: s2decl.clone(),
                                file_id: file_decl.file_id.clone(),
                                span: m.key.span.clone(),
                            }
                            .into(),
                        );
                        file_schema2_decls.push(s2decl);
                        macro_decls.push(decl.into());
                    }
                    Declaration::TypeDeclaration(ref t) => {
                        let decl = Schema2TypeDecl {
                            name: t.key.name.clone(),
                            arity: t.key.arity,
                            module: module.clone(),
                            app: app.clone(),
                        };
                        let s2decl = Schema2Declaration::Type(decl.clone().into());
                        decl_locations.push(
                            Schema2DeclLocation {
                                declaration: s2decl.clone(),
                                file_id: file_decl.file_id.clone(),
                                span: t.key.span.clone(),
                            }
                            .into(),
                        );
                        type_defs.push(
                            Schema2TypeDef {
                                declaration: decl.clone().into(),
                                exported: t.key.exported,
                                opaque: false,
                                definition_text: None,
                            }
                            .into(),
                        );
                        file_schema2_decls.push(s2decl);
                        type_decls.push(decl.into());
                    }
                    Declaration::RecordDeclaration(ref r) => {
                        let decl = Schema2RecordDecl {
                            name: r.key.name.clone(),
                            module: module.clone(),
                            app: app.clone(),
                        };
                        let s2decl = Schema2Declaration::Record(decl.clone().into());
                        decl_locations.push(
                            Schema2DeclLocation {
                                declaration: s2decl.clone(),
                                file_id: file_decl.file_id.clone(),
                                span: r.key.span.clone(),
                            }
                            .into(),
                        );
                        file_schema2_decls.push(s2decl);
                        record_decls.push(decl.into());
                    }
                    Declaration::HeaderDeclaration(ref h) => {
                        let decl = Schema2HeaderDecl {
                            name: h.key.name.clone(),
                            app: app.clone(),
                        };
                        let s2decl = Schema2Declaration::Header(decl.clone().into());
                        decl_locations.push(
                            Schema2DeclLocation {
                                declaration: s2decl.clone(),
                                file_id: file_decl.file_id.clone(),
                                span: h.key.span.clone(),
                            }
                            .into(),
                        );
                        file_schema2_decls.push(s2decl);
                        header_decls.push(decl.into());
                    }
                    Declaration::VarDeclaration(ref v) => {
                        let decl = Schema2VarDecl {
                            name: v.key.name.clone(),
                            module: module.clone(),
                            app: app.clone(),
                            span_start: v.key.span.start,
                            type_text: None,
                        };
                        var_locations.push(
                            Schema2VarLocation {
                                var_: decl.clone().into(),
                                file_id: file_decl.file_id.clone(),
                                span: v.key.span.clone(),
                            }
                            .into(),
                        );
                        var_decls.push(decl.into());
                    }
                    Declaration::DocDeclaration(ref doc) => {
                        // Convert doc to comment fact linked to the target declaration
                        if let Some(s2target) = self.decl_to_schema2(&doc.key.target, module, app) {
                            comments.push(
                                Schema2CommentFact {
                                    declaration: s2target,
                                    file_id: file_decl.file_id.clone(),
                                    span: doc.key.span.clone(),
                                    text: Some(doc.key.text.clone()),
                                }
                                .into(),
                            );
                        }
                    }
                }
            }

            file_decls_list.push(
                Schema2FileDeclarations {
                    file_id: file_decl.file_id.clone(),
                    declarations: file_schema2_decls,
                }
                .into(),
            );
        }

        // Convert xrefs to typed xrefs
        let xref_files = mem::take(&mut self.xrefs);
        let mut typed_xrefs: Vec<Key<Schema2XRefsByFile>> = vec![];
        let mut var_xrefs: Vec<Key<Schema2VarXRefsByFile>> = vec![];
        for xref_file in xref_files {
            let mut file_typed: Vec<Schema2XRef> = vec![];
            let mut var_map: FxHashMap<(String, u32), Vec<Location>> = FxHashMap::default();
            for xref in xref_file.xrefs {
                match &xref.target {
                    XRefTarget::Function(f) => {
                        let target_module = modules.get(&f.key.file_id).unwrap_or(&unknown);
                        let target_app = apps.get(&f.key.file_id).unwrap_or(&unknown);
                        file_typed.push(Schema2XRef {
                            target: Schema2Declaration::Func(
                                Schema2FuncDecl {
                                    fqn: Schema2Fqn {
                                        module: target_module.clone(),
                                        name: f.key.name.clone(),
                                        arity: f.key.arity,
                                    },
                                    app: target_app.clone(),
                                }
                                .into(),
                            ),
                            source: xref.source,
                        });
                    }
                    XRefTarget::Macro(m) => {
                        let target_module = modules.get(&m.key.file_id).unwrap_or(&unknown);
                        let target_app = apps.get(&m.key.file_id).unwrap_or(&unknown);
                        file_typed.push(Schema2XRef {
                            target: Schema2Declaration::Macro(
                                Schema2MacroDecl {
                                    name: m.key.name.clone(),
                                    arity: m.key.arity,
                                    module: target_module.clone(),
                                    app: target_app.clone(),
                                }
                                .into(),
                            ),
                            source: xref.source,
                        });
                    }
                    XRefTarget::Header(h) => {
                        let target_app = apps.get(&h.key.file_id).unwrap_or(&unknown);
                        file_typed.push(Schema2XRef {
                            target: Schema2Declaration::Header(
                                Schema2HeaderDecl {
                                    name: h.key.name.clone(),
                                    app: target_app.clone(),
                                }
                                .into(),
                            ),
                            source: xref.source,
                        });
                    }
                    XRefTarget::Record(r) => {
                        let target_module = modules.get(&r.key.file_id).unwrap_or(&unknown);
                        let target_app = apps.get(&r.key.file_id).unwrap_or(&unknown);
                        file_typed.push(Schema2XRef {
                            target: Schema2Declaration::Record(
                                Schema2RecordDecl {
                                    name: r.key.name.clone(),
                                    module: target_module.clone(),
                                    app: target_app.clone(),
                                }
                                .into(),
                            ),
                            source: xref.source,
                        });
                    }
                    XRefTarget::Type(t) => {
                        let target_module = modules.get(&t.key.file_id).unwrap_or(&unknown);
                        let target_app = apps.get(&t.key.file_id).unwrap_or(&unknown);
                        file_typed.push(Schema2XRef {
                            target: Schema2Declaration::Type(
                                Schema2TypeDecl {
                                    name: t.key.name.clone(),
                                    arity: t.key.arity,
                                    module: target_module.clone(),
                                    app: target_app.clone(),
                                }
                                .into(),
                            ),
                            source: xref.source,
                        });
                    }
                    XRefTarget::Var(v) => {
                        let span_start = v.key.decl_span_start.unwrap_or(xref.source.start);
                        let key = (v.key.name.clone(), span_start);
                        var_map.entry(key).or_default().push(xref.source);
                    }
                }
            }
            if !file_typed.is_empty() {
                typed_xrefs.push(
                    Schema2XRefsByFile {
                        file_id: xref_file.file_id.clone(),
                        xrefs: file_typed,
                    }
                    .into(),
                );
            }
            if !var_map.is_empty() {
                let file_vars: Vec<Schema2VarXRef> = var_map
                    .into_iter()
                    .map(|((name, span_start), sources)| Schema2VarXRef {
                        target_name: name,
                        target_span_start: span_start,
                        sources,
                    })
                    .collect();
                var_xrefs.push(
                    Schema2VarXRefsByFile {
                        file_id: xref_file.file_id,
                        xrefs: file_vars,
                    }
                    .into(),
                );
            }
        }

        // Convert module facts to erlang.2 Module + ModuleDefinition
        let module_facts = mem::take(&mut self.module_facts);
        let mut module2_decls: Vec<Key<Schema2ModuleDecl>> = vec![];
        let mut module2_defs: Vec<Key<Schema2ModuleDef>> = vec![];
        let mut file_includes: Vec<Key<Schema2FileIncludes>> = vec![];
        for mf in module_facts {
            let app = apps.get(&mf.file_id).unwrap_or(&unknown);
            let decl = Schema2ModuleDecl {
                file_id: mf.file_id.clone(),
                name: mf.name.clone(),
                app: app.clone(),
            };
            let exports: Vec<Key<Schema2FuncDecl>> = mf
                .exports
                .unwrap_or_default()
                .into_iter()
                .filter_map(|e| {
                    // exports are "name/arity" strings
                    let parts: Vec<&str> = e.splitn(2, '/').collect();
                    if parts.len() == 2 {
                        Some(
                            Schema2FuncDecl {
                                fqn: Schema2Fqn {
                                    module: mf.name.clone(),
                                    name: parts[0].to_string(),
                                    arity: parts[1].parse().unwrap_or(0),
                                },
                                app: app.clone(),
                            }
                            .into(),
                        )
                    } else {
                        None
                    }
                })
                .collect();
            file_includes.extend(mf.included_files.iter().map(|inc| {
                Schema2FileIncludes {
                    file_id: mf.file_id.clone(),
                    included: inc.clone(),
                }
                .into()
            }));
            let (cb_defs, cb_decls): (Vec<_>, Vec<_>) = mf
                .callbacks
                .iter()
                .map(|cb| {
                    let cb_decl = Schema2CallbackDecl {
                        name: cb.name.clone(),
                        arity: cb.arity,
                        module: mf.name.clone(),
                        app: app.clone(),
                    };
                    let cb_def = Schema2CallbackDef {
                        declaration: cb_decl.clone().into(),
                        optional_: cb.optional,
                    }
                    .into();
                    (cb_def, cb_decl.into())
                })
                .unzip();
            callback_defs.extend(cb_defs);
            callback_decls.extend(cb_decls);
            module2_defs.push(
                Schema2ModuleDef {
                    declaration: decl.clone().into(),
                    oncall: mf.oncall,
                    exports,
                    behaviours: mf.behaviours.unwrap_or_default(),
                    exdoc_link: mf.exdoc_link,
                    compile_options: mf.compile_options,
                }
                .into(),
            );
            module2_decls.push(decl.into());
        }

        vec![
            Fact::FuncDecl2 { facts: func_decls },
            Fact::MacroDecl2 { facts: macro_decls },
            Fact::RecordDecl2 {
                facts: record_decls,
            },
            Fact::TypeDecl2 { facts: type_decls },
            Fact::HeaderDecl2 {
                facts: header_decls,
            },
            Fact::CallbackDecl2 {
                facts: callback_decls,
            },
            Fact::RecordFieldDecl2 {
                facts: record_field_decls,
            },
            Fact::Module2 {
                facts: module2_decls,
            },
            Fact::VarDecl2 { facts: var_decls },
            Fact::FuncDef2 { facts: func_defs },
            Fact::MacroDef2 { facts: macro_defs },
            Fact::RecordDef2 { facts: record_defs },
            Fact::TypeDef2 { facts: type_defs },
            Fact::CallbackDef2 {
                facts: callback_defs,
            },
            Fact::ModuleDef2 {
                facts: module2_defs,
            },
            Fact::DeclLocation2 {
                facts: decl_locations,
            },
            Fact::VarLocation2 {
                facts: var_locations,
            },
            Fact::TypedXRefs2 { facts: typed_xrefs },
            Fact::VarXRefs2 { facts: var_xrefs },
            Fact::FileDecls2 {
                facts: file_decls_list,
            },
            Fact::DeclComment2 { facts: comments },
            Fact::FileIncludes2 {
                facts: file_includes,
            },
        ]
    }

    /// Helper: convert internal Declaration to Schema2Declaration
    fn decl_to_schema2(
        &self,
        decl: &Declaration,
        module: &str,
        app: &str,
    ) -> Option<Schema2Declaration> {
        match decl {
            Declaration::FunctionDeclaration(f) => Some(Schema2Declaration::Func(
                Schema2FuncDecl {
                    fqn: Schema2Fqn {
                        module: module.to_string(),
                        name: f.key.name.clone(),
                        arity: f.key.arity,
                    },
                    app: app.to_string(),
                }
                .into(),
            )),
            Declaration::MacroDeclaration(m) => Some(Schema2Declaration::Macro(
                Schema2MacroDecl {
                    name: m.key.name.clone(),
                    arity: m.key.arity,
                    module: module.to_string(),
                    app: app.to_string(),
                }
                .into(),
            )),
            Declaration::RecordDeclaration(r) => Some(Schema2Declaration::Record(
                Schema2RecordDecl {
                    name: r.key.name.clone(),
                    module: module.to_string(),
                    app: app.to_string(),
                }
                .into(),
            )),
            Declaration::TypeDeclaration(t) => Some(Schema2Declaration::Type(
                Schema2TypeDecl {
                    name: t.key.name.clone(),
                    arity: t.key.arity,
                    module: module.to_string(),
                    app: app.to_string(),
                }
                .into(),
            )),
            Declaration::HeaderDeclaration(h) => Some(Schema2Declaration::Header(
                Schema2HeaderDecl {
                    name: h.key.name.clone(),
                    app: app.to_string(),
                }
                .into(),
            )),
            _ => None,
        }
    }
}

/// Metrics collected during a Glean indexing run.
#[derive(Debug, serde::Serialize)]
pub struct IndexerMetrics {
    pub duration_ms: u64,
    pub file_count: usize,
    pub module_count: usize,
    pub entity_count: usize,
    pub xref_count: usize,
    pub output_bytes: u64,
    pub files_errored: usize,
    pub success: bool,
    pub error_message: Option<String>,
}

impl IndexerMetrics {
    pub(crate) fn from_facts(
        facts: &FxHashMap<String, IndexedFacts>,
        files_errored: usize,
    ) -> Self {
        let mut file_count = 0;
        let mut module_count = 0;
        let mut entity_count = 0;
        let mut xref_count = 0;

        for indexed in facts.values() {
            file_count += indexed.file_facts.len();
            module_count += indexed.module_facts.len();
            entity_count += indexed
                .file_declarations
                .iter()
                .map(|fd| fd.declarations.len())
                .sum::<usize>();
            xref_count += indexed.xrefs.iter().map(|f| f.xrefs.len()).sum::<usize>();
        }

        Self {
            duration_ms: 0,
            file_count,
            module_count,
            entity_count,
            xref_count,
            output_bytes: 0,
            files_errored,
            success: true,
            error_message: None,
        }
    }

    pub(crate) fn failed(duration_ms: u64, error: &anyhow::Error) -> Self {
        Self {
            duration_ms,
            file_count: 0,
            module_count: 0,
            entity_count: 0,
            xref_count: 0,
            output_bytes: 0,
            files_errored: 0,
            success: false,
            error_message: Some(format!("{error:#}")),
        }
    }
}
