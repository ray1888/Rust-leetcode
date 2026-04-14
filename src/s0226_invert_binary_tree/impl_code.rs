use std::cell::RefCell;
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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            node.borrow_mut().left = Solution::invert_tree(right);
            node.borrow_mut().right = Solution::invert_tree(left);
        }
        root
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
        let mut queue = std::collections::VecDeque::new();
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

    fn tree_to_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        if let Some(r) = root {
            queue.push_back(r);
        }
        while !queue.is_empty() {
            let num = queue.len();
            let mut level = Vec::new();
            for _ in 0..num {
                let node = queue.pop_front().unwrap();
                level.push(node.borrow().val);
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();
                if let Some(l) = left {
                    queue.push_back(l);
                }
                if let Some(r) = right {
                    queue.push_back(r);
                }
            }
            ans.push(level);
        }
        ans
    }

    #[test]
    fn test_invert_binary_tree() {
        let root = build_tree(&[
            Some(4),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            Some(6),
            Some(9),
        ]);
        let inverted = Solution::invert_tree(root);
        assert_eq!(
            tree_to_level_order(inverted),
            vec![vec![4], vec![7, 2], vec![9, 6, 3, 1]]
        );
    }

    #[test]
    fn test_invert_binary_tree_empty() {
        let root: Option<Rc<RefCell<TreeNode>>> = None;
        let inverted = Solution::invert_tree(root);
        assert!(inverted.is_none());
    }
}
