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
    fn preorder(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        match node {
            None => return,
            Some(t1) => {
                result.push(t1.borrow().val);
                Solution::preorder(&t1.borrow().left, result);
                Solution::preorder(&t1.borrow().right, result);
            }
        }
    }

    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut result: Vec<i32> = vec![];
        Solution::preorder(&root, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

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
    fn test_preorder_traversal_empty() {
        assert_eq!(Solution::preorder_traversal(None), Vec::<i32>::new());
    }

    #[test]
    fn test_preorder_traversal_standard() {
        // [1,null,2,3] -> preorder [1,2,3]
        let root = build_tree(&[Some(1), None, Some(2), Some(3)]);
        assert_eq!(Solution::preorder_traversal(root), vec![1, 2, 3]);
    }

    #[test]
    fn test_preorder_traversal_complex() {
        // [3,9,20,null,null,15,7] -> preorder [3,9,20,15,7]
        let root = build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(Solution::preorder_traversal(root), vec![3, 9, 20, 15, 7]);
    }
}
