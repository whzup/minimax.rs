use std::cell::RefCell;
use std::rc::Rc;

pub type NodeRef = Rc<RefCell<Node>>;
pub type NodeOption = Option<NodeRef>;

#[derive(PartialEq, Debug)]
pub struct Node {
    val: i32,
    color: bool,
    pub left: NodeOption,
    pub right: NodeOption,
}

impl Node {
    pub fn new(val: i32, color: bool, left: NodeOption, right: NodeOption) -> NodeRef {
        Rc::new(RefCell::new(Node {
            val: val,
            color: color,
            left: left,
            right: right
        }))
    }

    pub fn is_left_none(&self) -> bool {
        self.left.is_none()
    }

    pub fn is_right_none(&self) -> bool {
        self.right.is_none()
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    pub fn get_val(&self) -> i32 {
        self.val
    }

    pub fn set_val(&mut self, new_val : i32) {
        self.val = new_val;
    }

    pub fn remove_right(&mut self) {
        self.right = None;
    }

    pub fn remove_left(&mut self) {
        self.left = None;
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Node with value {} and color {} just dropped", self.val, self.color);
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_new_node() {
        let node = Node::new(12, true, None, None);
        assert_eq!(node, Rc::new(RefCell::new(Node { val: 12, color: true, left: None, right:
            None})));
    }
}
