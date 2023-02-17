#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut parent = root.unwrap();
        let rightChild = parent.borrow_mut().right.take();
        let leftChild = parent.borrow_mut().left.take();
        if rightChild.is_some() && leftChild.is_some() {
            return cmp::min(
                Solution::min_depth(rightChild),
                Solution::min_depth(leftChild),
            ) + 1;
        } else if rightChild.is_some() {
            return Solution::min_depth(rightChild) + 1;
        } else if leftChild.is_some() {
            return Solution::min_depth(leftChild) + 1;
        } else {
            return 1;
        }
    }
}
