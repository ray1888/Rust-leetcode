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
    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        match node {
            None => return,
            Some(t1) => {
                Solution::inorder(&t1.borrow().left, result);
                result.push(t1.borrow().val);
                Solution::inorder(&t1.borrow().right, result);
            }
        }
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut result: Vec<i32> = vec![];
        Solution::inorder(&root, &mut result);
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
    fn test_inorder_traversal_empty() {
        assert_eq!(Solution::inorder_traversal(None), Vec::<i32>::new());
    }

    #[test]
    fn test_inorder_traversal_standard() {
        // [1,null,2,3] -> inorder [1,3,2]
        let root = build_tree(&[Some(1), None, Some(2), Some(3)]);
        assert_eq!(Solution::inorder_traversal(root), vec![1, 3, 2]);
    }

    #[test]
    fn test_inorder_traversal_complex() {
        // [3,9,20,null,null,15,7] -> inorder [9,3,15,20,7]
        let root = build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(Solution::inorder_traversal(root), vec![9, 3, 15, 20, 7]);
    }
}
