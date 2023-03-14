pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn check(t1: &Option<Rc<RefCell<TreeNode>>>, t2: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (t1, t2) {
                (None, None) => true,
                (Some(n1), Some(n2)) => n1.borrow().val == n2.borrow().val && check(&n1.borrow().left, &n2.borrow().right) && check(&n1.borrow().right, &n2.borrow().left),
                _ => false
            }
        }
        root.is_none() || check(&root.as_ref().unwrap().borrow().left, &root.as_ref().unwrap().borrow().right)
    }
}