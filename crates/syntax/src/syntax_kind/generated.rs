//! @generated file, do not edit by hand, see `xtask/src/codegen.rs`

#![allow(bad_style, missing_docs, unreachable_pub)]
use num_derive::{FromPrimitive, ToPrimitive};
#[doc = r" The kind of syntax node, e.g. `ATOM`, `IF_KW`, or `DOT`."]
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
    FromPrimitive,
    ToPrimitive
)]
#[repr(u16)]
pub enum SyntaxKind {
    ANON_AFTER = 107u16,
    ANON_AND = 117u16,
    ANON_ANDALSO = 87u16,
    ANN_TYPE = 203u16,
    ANN_VAR = 204u16,
    ANON_RECORD_EXPR = 261u16,
    ANON_RECORD_FIELD_EXPR = 263u16,
    ANON_RECORD_UPDATE_EXPR = 262u16,
    ANONYMOUS_FUN = 280u16,
    ARITY = 283u16,
    ATOM = 1u16,
    ATTR_NAME = 198u16,
    B_GENERATOR = 241u16,
    ANON_BAND = 116u16,
    ANON_BANG = 85u16,
    ANON_BEGIN = 88u16,
    ANON_BEHAVIOR = 35u16,
    ANON_BEHAVIOUR = 33u16,
    BEHAVIOUR_ATTRIBUTE = 164u16,
    BIN_ELEMENT = 226u16,
    BINARY = 225u16,
    BINARY_COMPREHENSION = 235u16,
    BINARY_OP_EXPR = 217u16,
    BIT_SIZE_EXPR = 227u16,
    BIT_TYPE_LIST = 228u16,
    BIT_TYPE_UNIT = 233u16,
    BLOCK_EXPR = 223u16,
    ANON_BNOT = 112u16,
    ANON_BOR = 118u16,
    ANON_BSL = 120u16,
    ANON_BSR = 121u16,
    ANON_BXOR = 119u16,
    CALL = 269u16,
    CALLBACK = 193u16,
    ANON_CALLBACK = 74u16,
    ANON_CASE = 104u16,
    CASE_EXPR = 272u16,
    ANON_CATCH = 82u16,
    CATCH_CLAUSE = 289u16,
    CATCH_EXPR = 214u16,
    CHAR = 141u16,
    CLAUSE_BODY = 212u16,
    ANON_COLON = 4u16,
    ANON_COLON_COLON = 68u16,
    ANON_COLON_EQ = 102u16,
    ANON_COMMA = 30u16,
    COMMENT = 142u16,
    ANON_COMPILE = 51u16,
    COMPILE_OPTIONS_ATTRIBUTE = 173u16,
    CONCATABLES = 315u16,
    COND_MATCH_EXPR = 216u16,
    CR_CLAUSE = 275u16,
    ANON_D_AMP = 96u16,
    ANON_DASH = 7u16,
    ANON_DASH_DASH = 125u16,
    ANON_DASH_GT = 77u16,
    ANON_DEFINE = 28u16,
    ANON_DEPRECATED = 55u16,
    DEPRECATED_ATTRIBUTE = 175u16,
    DEPRECATED_FA = 180u16,
    DEPRECATED_FAS = 179u16,
    DEPRECATED_MODULE = 178u16,
    DEPRECATED_WILDCARD = 61u16,
    DEPRECATION_DESC = 181u16,
    ANON_DIV = 114u16,
    ANON_DOT = 2u16,
    ANON_DOT_DOT = 80u16,
    DOTDOTDOT = 81u16,
    ANON_ELIF = 26u16,
    ANON_ELSE = 20u16,
    ANON_END = 89u16,
    ANON_ENDIF = 22u16,
    ANON_EQ = 83u16,
    ANON_EQ_COLON_EQ = 132u16,
    ANON_EQ_EQ = 126u16,
    ANON_EQ_GT = 101u16,
    ANON_EQ_LT = 128u16,
    ANON_EQ_SLASH_EQ = 133u16,
    ANON_EXPORT = 37u16,
    EXPORT_ATTRIBUTE = 165u16,
    ANON_EXPORT_RECORD = 49u16,
    EXPORT_RECORD_ATTRIBUTE = 172u16,
    ANON_EXPORT_TYPE = 47u16,
    EXPORT_TYPE_ATTRIBUTE = 171u16,
    EXPR_ARGS = 311u16,
    EXTERNAL_FUN = 279u16,
    FA = 170u16,
    ANON_FEATURE = 57u16,
    FEATURE_ATTRIBUTE = 176u16,
    FIELD_EXPR = 267u16,
    FIELD_TYPE = 268u16,
    ANON_FILE = 53u16,
    FILE_ATTRIBUTE = 174u16,
    FLOAT = 137u16,
    ANON_FUN = 79u16,
    FUN_CLAUSE = 285u16,
    FUN_DECL = 199u16,
    FUN_TYPE = 206u16,
    FUN_TYPE_SIG = 207u16,
    FUNCTION_CLAUSE = 210u16,
    GENERATOR = 240u16,
    ANON_GT = 131u16,
    ANON_GT_EQ = 130u16,
    ANON_GT_GT = 91u16,
    GUARD = 313u16,
    GUARD_CLAUSE = 314u16,
    ANON_IF = 24u16,
    IF_CLAUSE = 271u16,
    IF_EXPR = 270u16,
    ANON_IFDEF = 16u16,
    ANON_IFNDEF = 18u16,
    ANON_IMPORT = 41u16,
    IMPORT_ATTRIBUTE = 166u16,
    ANON_IMPORT_RECORD = 43u16,
    IMPORT_RECORD_ATTRIBUTE = 167u16,
    IMPORT_RECORD_NAMES = 168u16,
    ANON_INCLUDE = 8u16,
    ANON_INCLUDE_LIB = 12u16,
    INTEGER = 136u16,
    INTERNAL_FUN = 278u16,
    ANON_LBRACE = 59u16,
    ANON_LBRACK = 39u16,
    LC_EXPRS = 237u16,
    LC_OR_ZC_EXPR = 238u16,
    LIST = 224u16,
    LIST_COMPREHENSION = 234u16,
    ANON_LPAREN = 10u16,
    ANON_LT = 129u16,
    ANON_LT_COLON_DASH = 98u16,
    ANON_LT_COLON_EQ = 100u16,
    ANON_LT_DASH = 97u16,
    ANON_LT_EQ = 99u16,
    ANON_LT_LT = 90u16,
    MACRO_CALL_ARGS = 307u16,
    MACRO_CALL_EXPR = 306u16,
    MACRO_EXPR = 310u16,
    MACRO_LHS = 304u16,
    MACRO_STRING = 309u16,
    MAP_COMPREHENSION = 236u16,
    MAP_EXPR = 247u16,
    MAP_EXPR_UPDATE = 246u16,
    MAP_FIELD = 249u16,
    MAP_GENERATOR = 242u16,
    MATCH_EXPR = 215u16,
    ANON_MAYBE = 109u16,
    MAYBE_EXPR = 295u16,
    MODULE = 196u16,
    ANON_MODULE = 31u16,
    MODULE_ATTRIBUTE = 163u16,
    MULTI_STRING = 183u16,
    NOMINAL = 187u16,
    ANON_NOMINAL = 64u16,
    ANON_NOT = 113u16,
    ANON_OF = 105u16,
    OPAQUE = 188u16,
    ANON_OPAQUE = 66u16,
    ANON_OPTIONAL_CALLBACKS = 45u16,
    OPTIONAL_CALLBACKS_ATTRIBUTE = 169u16,
    ANON_OR = 122u16,
    ANON_ORELSE = 86u16,
    PAREN_EXPR = 222u16,
    PIPE = 205u16,
    ANON_PIPE = 78u16,
    ANON_PIPE_PIPE = 95u16,
    ANON_PLUS = 111u16,
    ANON_PLUS_PLUS = 124u16,
    ANON_POUND = 71u16,
    ANON_POUND_UNDERSCORE = 103u16,
    PP_DEFINE = 161u16,
    PP_ELIF = 160u16,
    PP_ELSE = 157u16,
    PP_ENDIF = 158u16,
    PP_IF = 159u16,
    PP_IFDEF = 155u16,
    PP_IFNDEF = 156u16,
    PP_INCLUDE = 152u16,
    PP_INCLUDE_LIB = 153u16,
    PP_UNDEF = 154u16,
    ANON_QMARK = 110u16,
    ANON_QMARK_EQ = 84u16,
    QUALIFIED_RECORD_EXPR = 258u16,
    QUALIFIED_RECORD_FIELD_EXPR = 260u16,
    QUALIFIED_RECORD_NAME = 257u16,
    QUALIFIED_RECORD_UPDATE_EXPR = 259u16,
    RANGE_TYPE = 208u16,
    ANON_RBRACK = 40u16,
    ANON_RECEIVE = 106u16,
    RECEIVE_AFTER = 277u16,
    RECEIVE_EXPR = 276u16,
    ANON_RECORD = 69u16,
    RECORD_DECL = 191u16,
    RECORD_EXPR = 254u16,
    RECORD_FIELD = 266u16,
    RECORD_FIELD_EXPR = 252u16,
    RECORD_FIELD_NAME = 256u16,
    RECORD_INDEX_EXPR = 251u16,
    RECORD_NAME = 255u16,
    RECORD_UPDATE_EXPR = 253u16,
    ANON_REM = 115u16,
    REMOTE = 220u16,
    REMOTE_MODULE = 221u16,
    REPLACEMENT_CR_CLAUSES = 299u16,
    REPLACEMENT_EXPR_GUARD = 302u16,
    REPLACEMENT_FUNCTION_CLAUSES = 298u16,
    REPLACEMENT_GUARD_AND = 301u16,
    REPLACEMENT_GUARD_OR = 300u16,
    REPLACEMENT_PARENS = 303u16,
    ANON_RPAREN = 11u16,
    ANON_RRACE = 60u16,
    ANON_SEMI = 76u16,
    SHEBANG = 134u16,
    ANON_SLASH = 92u16,
    ANON_SLASH_EQ = 127u16,
    SOURCE_FILE = 146u16,
    SPEC = 192u16,
    ANON_SPEC = 72u16,
    ANON_SSR = 3u16,
    SSR_DEFINITION = 148u16,
    ANON_SSR_MATCH = 5u16,
    SSR_REPLACEMENT = 149u16,
    SSR_WHEN = 150u16,
    ANON_STAR = 93u16,
    STRING = 322u16,
    ANON_TRY = 108u16,
    TRY_AFTER = 288u16,
    TRY_CLASS = 290u16,
    TRY_EXPR = 286u16,
    TRY_STACK = 291u16,
    TUPLE = 245u16,
    ANON_TYPE = 62u16,
    TYPE_ALIAS = 186u16,
    TYPE_GUARDS = 202u16,
    TYPE_NAME = 190u16,
    TYPE_SIG = 201u16,
    UNARY_OP_EXPR = 218u16,
    ANON_UNDEF = 14u16,
    ANON_UNIT = 94u16,
    VAR = 135u16,
    VAR_ARGS = 312u16,
    ANON_WHEN = 6u16,
    WILD_ATTRIBUTE = 197u16,
    ANON_XOR = 123u16,
    WHITESPACE = 323u16,
    ERROR = u16::MAX,
}
use self::SyntaxKind::*;
impl SyntaxKind {
    #[allow(clippy::match_like_matches_macro)]
    pub fn is_keyword(&self) -> bool {
        match self {
            ANON_AFTER
            | ANON_AND
            | ANON_ANDALSO
            | ANON_BAND
            | ANON_BEGIN
            | ANON_BEHAVIOR
            | ANON_BEHAVIOUR
            | ANON_BNOT
            | ANON_BOR
            | ANON_BSL
            | ANON_BSR
            | ANON_BXOR
            | ANON_CALLBACK
            | ANON_CASE
            | ANON_CATCH
            | ANON_COMPILE
            | ANON_DEFINE
            | ANON_DEPRECATED
            | ANON_DIV
            | ANON_ELIF
            | ANON_ELSE
            | ANON_END
            | ANON_ENDIF
            | ANON_EXPORT
            | ANON_EXPORT_RECORD
            | ANON_EXPORT_TYPE
            | ANON_FEATURE
            | ANON_FILE
            | ANON_FUN
            | ANON_IF
            | ANON_IFDEF
            | ANON_IFNDEF
            | ANON_IMPORT
            | ANON_IMPORT_RECORD
            | ANON_INCLUDE
            | ANON_INCLUDE_LIB
            | ANON_MAYBE
            | ANON_MODULE
            | ANON_NOMINAL
            | ANON_NOT
            | ANON_OF
            | ANON_OPAQUE
            | ANON_OPTIONAL_CALLBACKS
            | ANON_OR
            | ANON_ORELSE
            | ANON_RECEIVE
            | ANON_RECORD
            | ANON_REM
            | ANON_SPEC
            | ANON_SSR
            | ANON_TRY
            | ANON_TYPE
            | ANON_UNDEF
            | ANON_UNIT
            | ANON_WHEN
            | ANON_XOR => true,
            _ => false,
        }
    }
    #[allow(clippy::match_like_matches_macro)]
    pub fn is_punct(&self) -> bool {
        match self {
            ANON_BANG
            | ANON_COLON
            | ANON_COLON_COLON
            | ANON_COLON_EQ
            | ANON_COMMA
            | ANON_D_AMP
            | ANON_DASH
            | ANON_DASH_DASH
            | ANON_DASH_GT
            | ANON_DOT
            | ANON_DOT_DOT
            | ANON_EQ
            | ANON_EQ_COLON_EQ
            | ANON_EQ_EQ
            | ANON_EQ_GT
            | ANON_EQ_LT
            | ANON_EQ_SLASH_EQ
            | ANON_GT
            | ANON_GT_EQ
            | ANON_GT_GT
            | ANON_LBRACE
            | ANON_LBRACK
            | ANON_LPAREN
            | ANON_LT
            | ANON_LT_COLON_DASH
            | ANON_LT_COLON_EQ
            | ANON_LT_DASH
            | ANON_LT_EQ
            | ANON_LT_LT
            | ANON_PIPE
            | ANON_PIPE_PIPE
            | ANON_PLUS
            | ANON_PLUS_PLUS
            | ANON_POUND
            | ANON_POUND_UNDERSCORE
            | ANON_QMARK
            | ANON_QMARK_EQ
            | ANON_RBRACK
            | ANON_RPAREN
            | ANON_RRACE
            | ANON_SEMI
            | ANON_SLASH
            | ANON_SLASH_EQ
            | ANON_SSR_MATCH
            | ANON_STAR => true,
            _ => false,
        }
    }
    #[allow(clippy::match_like_matches_macro)]
    pub fn is_literal(&self) -> bool {
        match self {
            ATOM | CHAR | COMMENT | DEPRECATED_WILDCARD | DOTDOTDOT | FLOAT | INTEGER | SHEBANG
            | VAR => true,
            _ => false,
        }
    }
    pub fn is_token(&self) -> bool {
        self.is_keyword() || self.is_punct() || self.is_literal()
    }
}
#[doc = r" Tell emacs to automatically reload this file if it changes"]
#[doc = r" Local Variables:"]
#[doc = r" auto-revert-mode: 1"]
#[doc = r" End:"]
fn _dummy() -> bool {
    false
}
