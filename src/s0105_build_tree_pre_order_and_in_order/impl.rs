pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;


impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build(&preorder, &inorder)
    }

    pub fn build(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));

        let mut index = 0;

        while index < inorder.len() {
            if inorder[index] == preorder[0] {
                break;
            }
            index += 1;
        }

        root.borrow_mut().left = Self::build(&preorder[1..=index], &inorder[0..index]);
        root.borrow_mut().right = Self::build(&preorder[index+1..], &inorder[index+1..]);

        Some(root)
    }
}

