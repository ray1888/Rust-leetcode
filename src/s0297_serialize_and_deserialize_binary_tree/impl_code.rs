use std::cell::RefCell;
use std::collections::VecDeque;
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

pub struct Codec;

impl Codec {
    pub fn new() -> Self {
        Codec
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return "[null]".to_string();
        }
        let mut result: Vec<String> = Vec::new();
        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let length = queue.len();
            for _ in 0..length {
                if let Some(node) = queue.pop_front() {
                    match node {
                        None => {
                            result.push("null".to_string());
                        }
                        Some(n) => {
                            result.push(n.borrow().val.to_string());
                            queue.push_back(n.borrow().left.clone());
                            queue.push_back(n.borrow().right.clone());
                        }
                    }
                }
            }
        }
        while let Some(last) = result.last() {
            if last == "null" {
                result.pop();
            } else {
                break;
            }
        }
        format!("[{}]", result.join(","))
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.len() <= 2 {
            return None;
        }
        let inner = &data[1..data.len() - 1];
        if inner.is_empty() {
            return None;
        }
        let ss: Vec<&str> = inner.split(',').collect();
        let mut nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::with_capacity(ss.len());
        for s in ss {
            if s == "null" {
                nodes.push(None);
            } else if let Ok(val) = s.parse::<i32>() {
                nodes.push(Some(Rc::new(RefCell::new(TreeNode::new(val)))));
            } else {
                nodes.push(None);
            }
        }
        let mut i = 1usize;
        for item in &nodes {
            if let Some(node) = item {
                if i < nodes.len() {
                    node.borrow_mut().left = nodes[i].clone();
                    i += 1;
                }
                if i < nodes.len() {
                    node.borrow_mut().right = nodes[i].clone();
                    i += 1;
                }
            }
        }
        nodes[0].clone()
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

    fn build_mintree() -> Option<Rc<RefCell<TreeNode>>> {
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

    fn trees_equal(a: &Option<Rc<RefCell<TreeNode>>>, b: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(n1), Some(n2)) => {
                n1.borrow().val == n2.borrow().val
                    && trees_equal(&n1.borrow().left, &n2.borrow().left)
                    && trees_equal(&n1.borrow().right, &n2.borrow().right)
            }
            _ => false,
        }
    }

    #[test]
    fn test_serialize_deserialize_symtree() {
        let codec = Codec::new();
        let root = build_symtree();
        let serialized = codec.serialize(root.clone());
        let deserialized = codec.deserialize(serialized);
        assert!(trees_equal(&root, &deserialized));
    }

    #[test]
    fn test_serialize_deserialize_nonsymtree() {
        let codec = Codec::new();
        let root = build_nonsymtree();
        let serialized = codec.serialize(root.clone());
        let deserialized = codec.deserialize(serialized);
        assert!(trees_equal(&root, &deserialized));
    }

    #[test]
    fn test_serialize_deserialize_mintree() {
        let codec = Codec::new();
        let root = build_mintree();
        let serialized = codec.serialize(root.clone());
        let deserialized = codec.deserialize(serialized);
        assert!(trees_equal(&root, &deserialized));
    }

    #[test]
    fn test_serialize_null() {
        let codec = Codec::new();
        let serialized = codec.serialize(None);
        assert_eq!(serialized, "[null]");
        let deserialized = codec.deserialize(serialized);
        assert!(deserialized.is_none());
    }
}
