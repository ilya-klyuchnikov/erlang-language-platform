/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is dual-licensed under either the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree or the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree. You may select, at your option, one of the
 * above-listed licenses.
 */

// Diagnostic: missing separator (W0004)
//
// Diagnostic for separators missing in function clauses.

use std::borrow::Cow;

use elp_ide_db::elp_base_db::FileRange;
use elp_syntax::ast::AstNode;
use elp_syntax::ast::ClauseSeparator;

use super::DiagnosticCode;
use super::GenericLinter;
use super::GenericLinterMatchContext;
use super::Linter;
use super::LinterContext;

pub(crate) struct MissingSeparatorLinter;

impl Linter for MissingSeparatorLinter {
    fn id(&self) -> DiagnosticCode {
        DiagnosticCode::MissingSeparator
    }

    fn description(&self) -> &'static str {
        "Missing separator in function clauses."
    }

    fn can_be_suppressed(&self) -> bool {
        false
    }

    fn should_process_generated_files(&self) -> bool {
        true
    }
}

impl GenericLinter for MissingSeparatorLinter {
    type Context = &'static str;

    fn matches(
        &self,
        ctx: &LinterContext,
    ) -> Option<Vec<GenericLinterMatchContext<Self::Context>>> {
        let mut res = Vec::new();
        let file_id = ctx.file_id;
        let def_map = ctx.sema.def_map(file_id);

        def_map.get_functions().for_each(|(_, fun_def)| {
            let n = fun_def.function_clauses.len();
            fun_def
                .function_clauses
                .iter()
                .enumerate()
                .for_each(|(i, fun)| {
                    if i + 1 != n {
                        // Non-last clause: expect ';'
                        match fun.separator {
                            Some((ClauseSeparator::Missing, range)) => {
                                res.push(GenericLinterMatchContext {
                                    range: FileRange { file_id, range },
                                    context: "Missing ';'",
                                });
                            }
                            None => {
                                let ast_fun = fun.form_id.get_ast(ctx.sema.db, file_id);
                                if let Some(last_tok) = ast_fun.syntax().last_token() {
                                    res.push(GenericLinterMatchContext {
                                        range: FileRange {
                                            file_id,
                                            range: last_tok.text_range(),
                                        },
                                        context: "Missing ';'",
                                    });
                                }
                            }
                            _ => {}
                        }
                    } else {
                        // Last clause: expect '.'
                        match fun.separator {
                            Some((ClauseSeparator::Missing, range)) => {
                                res.push(GenericLinterMatchContext {
                                    range: FileRange { file_id, range },
                                    context: "Missing '.'",
                                });
                            }
                            None => {
                                let ast_fun = fun.form_id.get_ast(ctx.sema.db, file_id);
                                if let Some(last_tok) = ast_fun.syntax().last_token() {
                                    res.push(GenericLinterMatchContext {
                                        range: FileRange {
                                            file_id,
                                            range: last_tok.text_range(),
                                        },
                                        context: "Missing '.'",
                                    });
                                }
                            }
                            _ => {}
                        }
                    }
                })
        });

        Some(res)
    }

    fn match_description(&self, context: &Self::Context) -> Cow<'_, str> {
        Cow::Borrowed(context)
    }
}

pub(crate) static LINTER: MissingSeparatorLinter = MissingSeparatorLinter;

// To run the tests via cargo
// cargo test --package elp_ide --lib
#[cfg(test)]
mod tests {

    use crate::tests::check_diagnostics;

    #[test]
    fn fun_decl_missing_semi_1() {
        check_diagnostics(
            r#"
   -module(main).
   foo(1)->2
        %% ^ warning: W0004: Missing ';'
   foo(2)->3.
"#,
        );
    }

    #[test]
    fn fun_decl_missing_semi_2() {
        check_diagnostics(
            r#"
   -module(main).
   foo(1)->2;
   foo(2)->3
        %% ^ warning: W0004: Missing ';'
   foo(3)->4.
"#,
        );
    }

    #[test]
    fn fun_decl_missing_dot_1() {
        check_diagnostics(
            r#"
   -module(main).
   foo(1)->2;
   foo(2)->3
        %% ^ warning: W0004: Missing '.'
"#,
        );
    }
}
