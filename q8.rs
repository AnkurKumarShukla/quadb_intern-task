// Given a binary tree, implement a function that returns the maximum depth of the tree.

use std::cmp::max;
use std::rc::Rc;
use std::cell::RefCell;

// #[derive(Debug, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left_depth = max_depth(node.borrow().left.clone());
            let right_depth = max_depth(node.borrow().right.clone());
            max(left_depth, right_depth) + 1
        }
    }
}


fn main() {
    let mut root = TreeNode::new(1);
    let mut left = TreeNode::new(2);
    let right = TreeNode::new(3);
    left.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    left.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));
    println!("{}", max_depth(Some(Rc::new(RefCell::new(root)))));
}
