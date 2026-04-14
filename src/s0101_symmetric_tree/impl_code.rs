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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn check(t1: &Option<Rc<RefCell<TreeNode>>>, t2: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (t1, t2) {
                (None, None) => true,
                (Some(n1), Some(n2)) => {
                    n1.borrow().val == n2.borrow().val
                        && check(&n1.borrow().left, &n2.borrow().right)
                        && check(&n1.borrow().right, &n2.borrow().left)
                }
                _ => false,
            }
        }
        root.is_none()
            || check(
                &root.as_ref().unwrap().borrow().left,
                &root.as_ref().unwrap().borrow().right,
            )
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

    /*
        1
       / \
      2   2
     / \ / \
    3  4 4  3
     */
    #[test]
    fn test_is_symmetric_true() {
        let success_input = build_tree(&[
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3),
        ]);
        assert!(Solution::is_symmetric(success_input));
    }

    /*
         1
       / \
      2   2
       \   \
       3    3
    */
    #[test]
    fn test_is_symmetric_false() {
        let failed_input = build_tree(&[Some(1), Some(2), Some(2), None, Some(3), None, Some(3)]);
        assert!(!Solution::is_symmetric(failed_input));
    }
}
