use crate::renderer::css::token::CssTokenizer;
use crate::renderer::css::token::CssToken;
use core::iter::Peekable;
use alloc::vec::Vec;
use alloc::string::String;

pub type ComponentValue = CssToken;

#[derive(Debug, Clone)]
pub struct CssParser {
    t: Peekable<CssTokenizer>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StyleSheet {
    // https://drafts.csswg.org/cssom/#dom-cssstylesheet-cssrules
    pub rules: Vec<QualifiedRule>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub property: String,
    pub value: ComponentValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QualifiedRule {
    // https://www.w3.org/TR/selectors-4/#typedef-selector-list
    pub selector: Selector,
    // https://www.w3.org/TR/css-syntax-3/#parse-a-list-of-declarations
    pub declarations: Vec<Declaration>,
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selector {
    TypeSelector(String),
    ClassSelector(String),
    IdSelector(String),
    UnknownSelector,
}

impl CssParser {
    pub fn new(t: CssTokenizer) -> Self {
        Self {t: t.peekable()}
    }
}

impl StyleSheet {
    pub fn new() -> Self {
        Self { rules: Vec::new()}
    }

    pub fn set_rules(&mut self, rules: Vec<QualifiedRule>) {
        self.rules = rules;
    }
}

impl QualifiedRule {
    pub fn new() -> Self {
        Self {
            selector: Selector::TypeSelector("".to_string()),
            declarations: Vec::new(),
        }
    }

    pub fn set_selector(&mut self, selector: Selector) {
        self.selector = selector;
    }

    pub fn set_declarations(&mut self, declarations: Vec<Declaration>) {
        self.declarations = declarations;
    }
}

impl Declaration {
    pub fn new() -> Self {
        Self {
            property: String::new(),
            value: ComponentValue::Ident(String::new()),
        }
    }

    pub fn set_property(&mut self, property: String) {
        self.property = property;
    }

    pub fn set_value(&mut self, value: ComponentValue) {
        self.value = value;
    }
}