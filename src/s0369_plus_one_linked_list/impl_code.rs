pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn plus_one(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut not_nine: *mut ListNode = &mut *dummy;
        let mut p: *mut ListNode = match dummy.next.as_mut() {
            Some(node) => &mut **node,
            None => std::ptr::null_mut(),
        };
        while !p.is_null() {
            unsafe {
                if (*p).val != 9 {
                    not_nine = p;
                }
                p = match (*p).next.as_mut() {
                    Some(node) => &mut **node,
                    None => std::ptr::null_mut(),
                };
            }
        }
        unsafe {
            (*not_nine).val += 1;
            p = match (*not_nine).next.as_mut() {
                Some(node) => &mut **node,
                None => std::ptr::null_mut(),
            };
            while !p.is_null() {
                (*p).val = 0;
                p = match (*p).next.as_mut() {
                    Some(node) => &mut **node,
                    None => std::ptr::null_mut(),
                };
            }
        }
        if dummy.val == 1 {
            Some(dummy)
        } else {
            dummy.next
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_list(vals: &[i32]) -> Option<Box<ListNode>> {
        if vals.is_empty() {
            return None;
        }
        let mut head = Box::new(ListNode::new(vals[0]));
        let mut cur = &mut head;
        for &v in &vals[1..] {
            cur.next = Some(Box::new(ListNode::new(v)));
            cur = cur.next.as_mut().unwrap();
        }
        Some(head)
    }

    fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut p = head;
        while let Some(node) = p {
            res.push(node.val);
            p = node.next;
        }
        res
    }

    #[test]
    fn test_plus_one() {
        assert_eq!(
            to_vec(Solution::plus_one(build_list(&[1, 2, 3]))),
            vec![1, 2, 4]
        );
        assert_eq!(
            to_vec(Solution::plus_one(build_list(&[1, 2, 9, 9]))),
            vec![1, 3, 0, 0]
        );
        assert_eq!(
            to_vec(Solution::plus_one(build_list(&[1, 2, 9]))),
            vec![1, 3, 0]
        );
        assert_eq!(
            to_vec(Solution::plus_one(build_list(&[9, 9, 9]))),
            vec![1, 0, 0, 0]
        );
    }
}
