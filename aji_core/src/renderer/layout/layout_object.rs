use crate::renderer::dom::node::Node;
use crate::renderer::layout::computed_style::ComputedStyle;
use alloc::rc::Rc;
use alloc::rc::Weak;
use core::cell::RefCell;

#[derive(Debug, Clone)]
pub struct LayoutObject {
    kind: LayoutObject,
    node: Rc<RefCell<Node>>,
    first_child: Option<Rc<RefCell<LayoutObject>>>,
    next_sibling: Option<Rc<RefCell<LayoutObject>>>,
    parent: Weak<RefCell<LayoutObject>>,
    style: ComputedStyle,
    point: LayoutPoint,
    size: LayoutSize,
}

impl LayoutObject {
    pub fn new(node: Rc<RefCell<Node>>, parent_obj: &Option<Rc<RefCell<LayoutObject>>>) -> Self {
        let parent = match parent_obj {
            Some(p) => Rc::downgrade(p),
            None => Weak::new(),
        };

        Self {
            kind: LatoutObjectKind::Block,
            node: node.clone(),
            first_child: None,
            next_sibling: None,
            parent,
            style: ComputedStyle::new(),
            point: LatoutPoint::new(0, 0),
            size: LayoutSize::new(0, 0),
        }
    }
}