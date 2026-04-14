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
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut cur = head;
        while let Some(mut cur_node) = cur {
            let next = cur_node.next.take();
            let mut p = &mut dummy as *mut Box<ListNode>;
            unsafe {
                while (*p).next.is_some() && (*p).next.as_ref().unwrap().val <= cur_node.val {
                    let next_p = &mut (*p).next as *mut Option<Box<ListNode>>;
                    p = (*next_p).as_mut().unwrap() as *mut Box<ListNode>;
                }
                cur_node.next = (*p).next.take();
                (*p).next = Some(cur_node);
            }
            cur = next;
        }
        dummy.next
    }

    pub fn insertion_sort_list_fast(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = None;
        let mut p: *mut Box<ListNode> = &mut dummy;
        let mut cur = head;
        while let Some(mut cur_node) = cur {
            let next = cur_node.next.take();
            unsafe {
                if (*p).next.is_some() && cur_node.val < (*p).next.as_ref().unwrap().val {
                    p = &mut dummy;
                }
                while (*p).next.is_some() && (*p).next.as_ref().unwrap().val <= cur_node.val {
                    let next_p = &mut (*p).next as *mut Option<Box<ListNode>>;
                    p = (*next_p).as_mut().unwrap() as *mut Box<ListNode>;
                }
                cur_node.next = (*p).next.take();
                (*p).next = Some(cur_node);
            }
            cur = next;
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort_list() {
        let input = build_list(&[1, 3, 2, 4]);
        let output = build_list(&[1, 2, 3, 4]);
        let res = Solution::insertion_sort_list(input);
        assert!(list_equal(&res, &output));
    }

    #[test]
    fn test_insertion_sort_list_fast() {
        let input = build_list(&[1, 3, 2, 4]);
        let output = build_list(&[1, 2, 3, 4]);
        let res = Solution::insertion_sort_list_fast(input);
        assert!(list_equal(&res, &output));
    }
}
