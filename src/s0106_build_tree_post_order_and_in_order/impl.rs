pub struct Solution;


use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if inorder.len() == 0 || postorder.len() == 0 {
                return None;
            }

            // println!("inorder: {:?}, postorder: {:?}", inorder, postorder);
            let mut root = TreeNode::new(*postorder.last().unwrap());

            if inorder.len() > 1 && postorder.len() > 1 {
                let root_val = root.val;
                let root_idx = inorder.iter().position(|x| *x == root_val).unwrap();

                root.left = helper(&inorder[..root_idx], &postorder[..root_idx]);
                root.right = helper(
                    &inorder[root_idx + 1..],
                    &postorder[root_idx..postorder.len() - 1],
                );
            }

            return Some(Rc::new(RefCell::new(root)));
        }

        helper(&inorder[..], &postorder[..])
    }
}
