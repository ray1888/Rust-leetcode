use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::borrow::BorrowMut;

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut ans = Vec::new();
    let mut stack = Vec::new();
    if root.is_none(){
        return ans;
    }
    stack.push(root.unwrap());
    while stack.is_empty()!= true{
        let num = stack.len();
        let mut level = Vec::new();
        for _i in 0..num{
            let tmp = stack.remove(0);
            level.push(tmp.borrow_mut().val);
            if tmp.borrow_mut().left.is_some(){
                stack.push(tmp.borrow_mut().left.take().unwrap());
            }
            if tmp.borrow_mut().right.is_some(){
                stack.push(tmp.borrow_mut().right.take().unwrap());
            }
        }
        ans.push(level);
    }
    ans
}