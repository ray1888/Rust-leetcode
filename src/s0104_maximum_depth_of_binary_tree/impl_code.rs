use std::cell::RefCell;
use std::cmp;
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let parent = root.unwrap();
        let right_child = parent.borrow_mut().right.take();
        let left_child = parent.borrow_mut().left.take();
        if right_child.is_some() && left_child.is_some() {
            cmp::max(
                Solution::max_depth(right_child),
                Solution::max_depth(left_child),
            ) + 1
        } else if right_child.is_some() {
            Solution::max_depth(right_child) + 1
        } else if left_child.is_some() {
            Solution::max_depth(left_child) + 1
        } else {
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_symtree() -> Option<Rc<RefCell<TreeNode>>> {
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let left = Rc::new(RefCell::new(TreeNode::new(2)));
        let right = Rc::new(RefCell::new(TreeNode::new(2)));
        let lleft = Rc::new(RefCell::new(TreeNode::new(3)));
        let lright = Rc::new(RefCell::new(TreeNode::new(4)));
        let rleft = Rc::new(RefCell::new(TreeNode::new(4)));
        let rright = Rc::new(RefCell::new(TreeNode::new(3)));
        left.borrow_mut().left = Some(lleft);
        left.borrow_mut().right = Some(lright);
        right.borrow_mut().left = Some(rleft);
        right.borrow_mut().right = Some(rright);
        root.borrow_mut().left = Some(left);
        root.borrow_mut().right = Some(right);
        Some(root)
    }

    fn build_nonsymtree() -> Option<Rc<RefCell<TreeNode>>> {
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let left = Rc::new(RefCell::new(TreeNode::new(2)));
        let right = Rc::new(RefCell::new(TreeNode::new(2)));
        let lleft = Rc::new(RefCell::new(TreeNode::new(3)));
        let llleft = Rc::new(RefCell::new(TreeNode::new(1)));
        let lright = Rc::new(RefCell::new(TreeNode::new(4)));
        let rleft = Rc::new(RefCell::new(TreeNode::new(4)));
        let rright = Rc::new(RefCell::new(TreeNode::new(3)));
        lleft.borrow_mut().left = Some(llleft);
        left.borrow_mut().left = Some(lleft);
        left.borrow_mut().right = Some(lright);
        right.borrow_mut().left = Some(rleft);
        right.borrow_mut().right = Some(rright);
        root.borrow_mut().left = Some(left);
        root.borrow_mut().right = Some(right);
        Some(root)
    }

    fn build_maxtree() -> Option<Rc<RefCell<TreeNode>>> {
        let root = Rc::new(RefCell::new(TreeNode::new(3)));
        let left = Rc::new(RefCell::new(TreeNode::new(9)));
        let right = Rc::new(RefCell::new(TreeNode::new(20)));
        let rleft = Rc::new(RefCell::new(TreeNode::new(15)));
        let rright = Rc::new(RefCell::new(TreeNode::new(7)));
        right.borrow_mut().left = Some(rleft);
        right.borrow_mut().right = Some(rright);
        root.borrow_mut().left = Some(left);
        root.borrow_mut().right = Some(right);
        Some(root)
    }

    #[test]
    fn test_max_depth() {
        let tests = vec![
            (build_symtree(), 3),
            (build_nonsymtree(), 4),
            (build_maxtree(), 3),
        ];
        for (root, expected) in tests {
            let actual = Solution::max_depth(root);
            assert_eq!(actual, expected);
        }
    }
}
