/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

//! Re-export of the canonical `MemoryUsage`/`Bytes` implementation from
//! `elp_ide_db`, kept here so existing `elp::memory_usage::*` import paths
//! remain valid for the binary crates and the LSP server.

pub use elp_ide_db::memory_usage::Bytes;
pub use elp_ide_db::memory_usage::MemoryUsage;
pub use elp_ide_db::memory_usage::memory_usage;
