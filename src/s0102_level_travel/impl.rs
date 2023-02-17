use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![vec![]];
        }
        let mut queue = VecDeque::new();
        let mut result: Vec<Vec<i32>> = Vec::new();
        queue.push_back(root.unwrap());
        let mut length = 0;
        while queue.len() > 0 {
            length = queue.len();
            let mut levelResult = Vec::new();
            for i in 0..length {
                let mut node = queue.pop_front();
                if node.is_none() {
                    continue;
                }
                let mut parent = node.unwrap();
                levelResult.push(parent.borrow_mut().val);
                let leftChild = parent.borrow_mut().left.take();
                let rightChild = parent.borrow_mut().right.take();
                if leftChild.is_some() {
                    queue.push_back(leftChild.unwrap());
                }
                if rightChild.is_some() {
                    queue.push_back(rightChild.unwrap());
                }
            }
            result.push(levelResult);
        }
        return result;
    }
}
