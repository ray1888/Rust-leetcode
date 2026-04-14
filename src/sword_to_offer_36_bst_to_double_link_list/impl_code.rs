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

#[derive(Debug, PartialEq, Eq)]
pub struct DoubleLinkListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<DoubleLinkListNode>>>,
    pub pre: Option<Rc<RefCell<DoubleLinkListNode>>>,
}

impl DoubleLinkListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        DoubleLinkListNode {
            val,
            next: None,
            pre: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn bst_to_double_link_list(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<DoubleLinkListNode>>> {
        let mut result: Vec<i32> = Vec::new();
        Solution::mid_travel(&root, &mut result);
        Solution::make_double_link_list(&result)
    }

    fn mid_travel(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        match node {
            None => return,
            Some(n) => {
                Solution::mid_travel(&n.borrow().left, result);
                result.push(n.borrow().val);
                Solution::mid_travel(&n.borrow().right, result);
            }
        }
    }

    fn make_double_link_list(arr: &[i32]) -> Option<Rc<RefCell<DoubleLinkListNode>>> {
        if arr.is_empty() {
            return None;
        }

        let head = Rc::new(RefCell::new(DoubleLinkListNode::new(arr[0])));
        let mut current = Rc::clone(&head);

        for i in 1..arr.len() {
            let new_node = Rc::new(RefCell::new(DoubleLinkListNode::new(arr[i])));
            new_node.borrow_mut().pre = Some(Rc::clone(&current));
            current.borrow_mut().next = Some(Rc::clone(&new_node));
            current = new_node;
        }

        Some(head)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst_to_double_link_list() {
        // Build BST: root 2, left 1, right 3
        let left = Rc::new(RefCell::new(TreeNode::new(1)));
        let right = Rc::new(RefCell::new(TreeNode::new(3)));
        let root = Rc::new(RefCell::new(TreeNode::new(2)));
        root.borrow_mut().left = Some(left);
        root.borrow_mut().right = Some(right);

        let head = Solution::bst_to_double_link_list(Some(root));
        assert!(head.is_some());

        // Traverse forward and verify values [1, 2, 3]
        let mut values = Vec::new();
        let mut current = head.clone();
        while let Some(node) = current {
            values.push(node.borrow().val);
            let next = node.borrow().next.as_ref().map(Rc::clone);
            current = next;
        }
        assert_eq!(values, vec![1, 2, 3]);

        // Traverse backward and verify values [3, 2, 1]
        let mut tail = head;
        let mut backward_values = Vec::new();
        while let Some(node) = tail {
            let next = node.borrow().next.as_ref().map(Rc::clone);
            if next.is_none() {
                // found tail, now go backward
                let mut back = Some(node);
                while let Some(n) = back {
                    backward_values.push(n.borrow().val);
                    back = n.borrow().pre.as_ref().map(Rc::clone);
                }
                break;
            }
            tail = next;
        }
        assert_eq!(backward_values, vec![3, 2, 1]);
    }
}
