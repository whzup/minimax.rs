use crate::tree_lib::node::{ Node, NodeOption };

#[derive(PartialEq, Debug)]
pub struct BTree {
    pub root: NodeOption,
    depth: u32,
}

impl BTree {
    pub fn new_empty() -> Self {
        BTree {
            root: None,
            depth: 0,
        }
    }

    pub fn new(val: i32, color: bool) -> Self {
        let new_root = Node::new(val, color, None, None);

        BTree {
            root: Some(new_root),
            depth: 1
        }
    }

    pub fn set_root(&mut self, root : NodeOption) {
        self.root = root
    }

    pub fn is_left_none(&self) -> bool {
        match &self.root {
            None => true,
            Some(root) => root.borrow().left.is_none()
        }
    }

    pub fn is_right_none(&self) -> bool {
        match &self.root {
            None => true,
            Some(root) => root.borrow().right.is_none()
        }
    }

    pub fn get_root_val(&self) -> Option<i32> {
        match &self.root {
            None => None,
            Some(node) => Some(node.borrow().get_val())
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_new_empty_tree() {
        let tree = BTree::new_empty();

        assert_eq!(tree, BTree {
            root: None,
            depth: 0
        });
    }

    #[test]
    fn test_new_tree() {
        let tree = BTree::new(12, true);

        assert_eq!(tree, BTree {
            root: Some(Node::new(12, true)),
            depth: 1
        })
    }
}
