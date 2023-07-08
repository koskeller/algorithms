use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
// Basic idea: on each iteration set constrains of min and max value for descendant nodes
// Time complexity: O(n)
// Space complixity: O(n) because of stack
pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_valid_fn(
        root.clone(),
        std::i32::MIN as i64 - 1,
        std::i32::MAX as i64 + 1,
    )
}

fn is_valid_fn(node: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
    match node {
        Some(node) => {
            let n = node.borrow();
            let value = n.val as i64;
            let left = is_valid_fn(n.left.clone(), min, value);
            let right = is_valid_fn(n.right.clone(), value, max);
            value > min && value < max && left && right
        }
        None => return true,
    }
}
