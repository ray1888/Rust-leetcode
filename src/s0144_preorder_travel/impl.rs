pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn preorder(node: &Option<Rc<RefCell<TreeNode>>>, result:&mut Vec<i32>) {
       match (node) {
           (None) => return,
           (Some(t1)) => {
                result.push(t1.borrow().val);
                Solution::preorder(&t1.borrow().left,  result);
                Solution::preorder(&t1.borrow().right,   result);
           }
       }
    }

    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none(){
            return vec![];
        }
        let mut result :Vec<i32>= vec![];
        Solution::preorder(&root, &mut result);
        return result;
    }
}