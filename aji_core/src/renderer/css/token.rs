use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub enum CssToken {
    // https://www.w3.org/TR/css-syntax-3/#typedef-hash-token
    HashToken(String),
    // https://www.w3.org/TR/css-syntax-3/#typedef-delim-token
    Delim(char),
    // https://www.w3.org/TR/css-syntax-3/#typedef-number-token
    Number(f64),
    // https://www.w3.org/TR/css-syntax-3/#typedef-colon-token
    Colon,
    // https://www.w3.org/TR/css-syntax-3/#typedef-semicolon-token
    SemiColon,
    // https://www.w3.org/TR/css-syntax-3/#tokendef-open-paren
    OpenParenthesis,
    // https://www.w3.org/TR/css-syntax-3/#tokendef-close-paren
    CloseParenthesis,
    // https://www.w3.org/TR/css-syntax-3/#tokendef-open-curly
    OpenCurly,
    // https://www.w3.org/TR/css-syntax-3/#tokendef-close-curly
    CloseCurly,
    // https://www.w3.org/TR/css-syntax-3/#typedef-ident-token
    Ident(String),
    // https://www.w3.org/TR/css-syntax-3/#typedef-string-token
    StringToken(String),
    // https://www.w3.org/TR/css-syntax-3/#typedef-at-keyword-token
    AtKeyword(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CssTokenizer {
    pos: usize,
    input: Vec<char>,
}

impl CssTokenizer {
    pub fn new(css: String) -> Self {
        Self {
            pos: 0,
            input: css.chars().collect(),
        }
    }
}
