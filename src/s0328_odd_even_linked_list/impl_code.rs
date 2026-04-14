// Definition for singly-linked list.
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

fn build_list(data: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &v in data.iter().rev() {
        let mut node = Box::new(ListNode::new(v));
        node.next = head;
        head = Some(node);
    }
    head
}

fn list_equal(l1: &Option<Box<ListNode>>, l2: &Option<Box<ListNode>>) -> bool {
    let mut p = l1.as_ref();
    let mut q = l2.as_ref();
    while let (Some(a), Some(b)) = (p, q) {
        if a.val != b.val {
            return false;
        }
        p = a.next.as_ref();
        q = b.next.as_ref();
    }
    p.is_none() && q.is_none()
}

pub struct Solution;

impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut odd = head.as_mut().unwrap() as *mut Box<ListNode>;
        let mut even_head = unsafe { (*odd).next.take() };
        let mut even = even_head.as_mut().map(|n| n as *mut Box<ListNode>);
        unsafe {
            while let Some(e) = even {
                if (*e).next.is_none() {
                    break;
                }
                let next_odd = (*e).next.take().unwrap();
                (*odd).next = Some(next_odd);
                odd = (*odd).next.as_mut().unwrap() as *mut Box<ListNode>;
                let next_even = (*odd).next.take();
                (*e).next = next_even;
                even = (*e).next.as_mut().map(|n| n as *mut Box<ListNode>);
            }
            (*odd).next = even_head;
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd_even_list_normal() {
        let input = build_list(&[1, 2, 3, 4]);
        let output = build_list(&[1, 3, 2, 4]);
        let res = Solution::odd_even_list(input);
        assert!(list_equal(&res, &output));
    }

    #[test]
    fn test_odd_even_list_double_nine() {
        let input = build_list(&[3, 6, 2, 8, 7]);
        let output = build_list(&[3, 2, 7, 6, 8]);
        let res = Solution::odd_even_list(input);
        assert!(list_equal(&res, &output));
    }
}
