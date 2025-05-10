use crate::browser::Browser;
use crate::renderer::dom::node::Window;
use alloc::rc::Rc;
use alloc::rc::Weak;
use core::cell::RefCell;
use crate::alloc::string::ToString;
use crate::http::HttpResponse;
use crate::renderer::html::parser::HtmlParser;
use crate::renderer::html::token::HtmlTokenizer;
use crate::utils::convert_dom_to_string;
use alloc::string::String;

#[derive(Debug, Clone)]
pub struct Page {
    pub browser: Weak<RefCell<Browser>>,
    frame: Option<Rc<RefCell<Window>>>,
}

impl Page {
    pub fn new() -> Self {
        Self {
            browser: Weak::new(),
            frame: None,
        }
    }

    pub fn set_browser(&mut self, browser: Weak<RefCell<Browser>>) {
        self.browser = browser;
    }

    pub fn receive_response(&mut self, html: String) {
        let html_tokenizer = HtmlTokenizer::new(html);
        let frame = HtmlParser::new(html_tokenizer).construct_tree();
        self.frame = Some(frame);
    }
}