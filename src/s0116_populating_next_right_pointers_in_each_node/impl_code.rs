use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
    pub next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node {
            val,
            left: None,
            right: None,
            next: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn connect(root: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        if root.is_none() {
            return None;
        }
        let mut leftmost = root.clone();
        while let Some(lm) = leftmost.clone() {
            let lm_ref = lm.borrow();
            if lm_ref.left.is_none() {
                break;
            }
            let mut p = leftmost.clone();
            while let Some(cur) = p.clone() {
                let cur_ref = cur.borrow();
                if let Some(left) = cur_ref.left.clone() {
                    left.borrow_mut().next = cur_ref.right.clone();
                }
                if let Some(next) = cur_ref.next.clone() {
                    if let Some(right) = cur_ref.right.clone() {
                        right.borrow_mut().next = next.borrow().left.clone();
                    }
                }
                p = cur_ref.next.clone();
            }
            leftmost = lm_ref.left.clone();
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_connect_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<Node>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }
        let root = Rc::new(RefCell::new(Node::new(vals[0].unwrap())));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;
        while i < vals.len() {
            let node = queue.pop_front().unwrap();
            if let Some(v) = vals[i] {
                let left = Rc::new(RefCell::new(Node::new(v)));
                node.borrow_mut().left = Some(left.clone());
                queue.push_back(left);
            }
            i += 1;
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let right = Rc::new(RefCell::new(Node::new(v)));
                    node.borrow_mut().right = Some(right.clone());
                    queue.push_back(right);
                }
                i += 1;
            }
        }
        Some(root)
    }

    fn nexts_to_vec(root: Option<Rc<RefCell<Node>>>) -> Vec<Option<i32>> {
        let mut result = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        if let Some(r) = root {
            queue.push_back(r);
        }
        while !queue.is_empty() {
            let num = queue.len();
            for _ in 0..num {
                let node = queue.pop_front().unwrap();
                let n = node.borrow();
                result.push(n.next.as_ref().map(|x| x.borrow().val));
                if let Some(left) = n.left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = n.right.clone() {
                    queue.push_back(right);
                }
            }
        }
        result
    }

    #[test]
    fn test_connect_right() {
        let root = build_connect_tree(&[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ]);
        let connected = Solution::connect(root);
        assert_eq!(
            nexts_to_vec(connected),
            vec![None, Some(3), None, Some(5), Some(6), Some(7), None]
        );
    }
}
