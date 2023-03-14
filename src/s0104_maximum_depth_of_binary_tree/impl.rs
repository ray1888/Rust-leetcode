pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max; 

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none(){
            return 0 
        }
        let mut parent = root.unwrap();
        let rightChild = parent.borrow_mut().right.take();
        let leftChild = parent.borrow_mut().left.take();
        if rightChild.is_some() && leftChild.is_some(){
            return max(Solution::max_depth(rightChild), Solution::max_depth(leftChild)) + 1;
        } else if rightChild.is_some(){
            return Solution::max_depth(rightChild) + 1;
        } else if leftChild.is_some(){
            return Solution::max_depth(leftChild) + 1;
        } else {
            return 1;
        }
    }
}