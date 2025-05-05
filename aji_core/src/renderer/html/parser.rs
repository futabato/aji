use crate::renderer::dom::node::Node;
use crate::renderer::dom::node::Window;
use crate::renderer::html::token::HtmlTokenizer;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::Ref;
use core::cell::RefCell;

#[derive(Debug, Clone)]
pub struct HtmlParser {
    window: Rc<RefCell<Window>>,
    model: InsertionMode,
    // https://html.spec.whatwg.org/multipage/parsing.html#original-insertion-mode
    original_insertion_mode: InsertionMode,
    // https://html.spec.whatwg.org/multipage/parsing.html#the-stack-of-open-elements
    stack_of_open_elements: Vec<Rc<RefCell<Node>>>,
    t: HtmlTokenizer,
}

impl HtmlParser {
    pub fn new(t: HtmlTokenizer) -> Self {
        Self {
            window: Rc::new(RefCell::new(Window::new())),
            model: InsertionMode::Initial,
            original_insertion_mode: InsertionMode::Initial,
            stack_of_open_elements: Vec::new(),
            t,
        }
    }
}

// https://html.spec.whatwg.org/multipage/parsing.html#original-insertion-mode
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InsertionMode {
    Initial,
    BeforeHtml,
    BeforeHead,
    InHead,
    AfterHead,
    InBody,
    Text,
    AfterBody,
    AfterAfterBody,
}

// pub fn construct_tree(&mut self) -> Rc<RefCell<Window>> {
//     let mut token = self.t.next();

//     while token.is_some() {
//         match self.mode {
//             InsertionMode::Initial => {}
//             InsertionMode::BeforeHtml => {}
//             InsertionMode::BeforeHead => {}
//             InsertionMode::InHead => {}
//             InsertionMode::AfterHead => {}
//             InsertionMode::InBody => {}
//             InsertionMode::Text => {}
//             InsertionMode::AfterBody => {}
//             InsertionMode::AfterAfterBody => {}
//         }
//     }
// }
