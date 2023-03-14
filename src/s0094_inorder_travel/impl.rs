pub struct Solution;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, result:&mut Vec<i32>) {
       match (node) {
           (None) => return,
           (Some(t1)) => {
                Solution::inorder(&t1.borrow().left,  result);
                result.push(t1.borrow().val);
                Solution::inorder(&t1.borrow().right,   result);
           }
       }
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none(){
            return vec![];
        }
        let mut result :Vec<i32>= vec![];
        Solution::inorder(&root, &mut result);
        return result;
    }
}