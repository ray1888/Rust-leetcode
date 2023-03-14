pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;


impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn check(t1: &Option<Rc<RefCell<TreeNode>>>, t2: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (t1, t2) {
                (None, None) => true,
                (Some(n1), Some(n2)) => n1.borrow().val == n2.borrow().val && check(&n1.borrow().left, &n2.borrow().left) && check(&n1.borrow().right, &n2.borrow().right),
                _ => false
            }
        }
        return check(&p, &q);
    }
}