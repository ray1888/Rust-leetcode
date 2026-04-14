use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

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

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut queue = VecDeque::new();
        let mut result: Vec<Vec<i32>> = Vec::new();
        queue.push_back(root.unwrap());
        while !queue.is_empty() {
            let length = queue.len();
            let mut level_result = Vec::new();
            for _ in 0..length {
                let parent = queue.pop_front().unwrap();
                let val = parent.borrow().val;
                level_result.push(val);
                let left = parent.borrow().left.clone();
                let right = parent.borrow().right.clone();
                if let Some(left) = left {
                    queue.push_back(left);
                }
                if let Some(right) = right {
                    queue.push_back(right);
                }
            }
            result.push(level_result);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;
        while i < vals.len() {
            let node = queue.pop_front().unwrap();
            if let Some(v) = vals[i] {
                let left = Rc::new(RefCell::new(TreeNode::new(v)));
                node.borrow_mut().left = Some(left.clone());
                queue.push_back(left);
            }
            i += 1;
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().right = Some(right.clone());
                    queue.push_back(right);
                }
                i += 1;
            }
        }
        Some(root)
    }

    #[test]
    fn test_level_order_empty() {
        assert_eq!(Solution::level_order(None), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_level_order_standard() {
        // [3,9,20,null,null,15,7]
        let root = build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(
            Solution::level_order(root),
            vec![vec![3], vec![9, 20], vec![15, 7]]
        );
    }

    #[test]
    fn test_level_order_symmetric() {
        // [1,2,2,3,4,4,3]
        let root = build_tree(&[
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3),
        ]);
        assert_eq!(
            Solution::level_order(root),
            vec![vec![1], vec![2, 2], vec![3, 4, 4, 3]]
        );
    }
}
