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
    pub fn reorder_list_iter_space_o1(head: &mut Option<Box<ListNode>>) {
        if head.is_none() {
            return;
        }
        let mut work: *mut Option<Box<ListNode>> = head;
        unsafe {
            while let Some(ref w) = *work {
                if w.next.is_none() || w.next.as_ref().unwrap().next.is_none() {
                    break;
                }
                // find slow and fast from work
                let mut slow: *mut Option<Box<ListNode>> = work;
                let mut fast: *mut Option<Box<ListNode>> = work;
                if let Some(_) = *fast {
                    fast = &mut (*fast).as_mut().unwrap().next as *mut Option<Box<ListNode>>;
                }
                while let Some(ref f) = *fast {
                    if f.next.is_none() {
                        break;
                    }
                    slow = &mut (*slow).as_mut().unwrap().next as *mut Option<Box<ListNode>>;
                    fast = &mut (*fast).as_mut().unwrap().next.as_mut().unwrap().next
                        as *mut Option<Box<ListNode>>;
                }
                // detach fast node
                let mut fast_node = (*slow).as_mut().unwrap().next.take();
                let tmp = (*work).as_mut().unwrap().next.take();
                (*work).as_mut().unwrap().next = fast_node.take();
                (*work).as_mut().unwrap().next.as_mut().unwrap().next = tmp;
                // break the tail after the inserted fast node to avoid cycles
                if let Some(ref mut w2) = (*work).as_mut().unwrap().next.as_mut().unwrap().next {
                    if let Some(ref fast_ref) = *fast {
                        if w2
                            .next
                            .as_ref()
                            .map_or(false, |n| std::ptr::eq(&**n, fast_ref.as_ref()))
                        {
                            w2.next = None;
                        }
                    }
                }
                work = &mut (*work).as_mut().unwrap().next.as_mut().unwrap().next
                    as *mut Option<Box<ListNode>>;
            }
        }
    }

    pub fn reorder_list_iter_space_on(head: &mut Option<Box<ListNode>>) {
        if head.is_none() {
            return;
        }
        let mut vals = Vec::new();
        {
            let mut p = head.as_ref();
            while let Some(node) = p {
                vals.push(node.val);
                p = node.next.as_ref();
            }
        }
        let mut new_head: Option<Box<ListNode>> = None;
        let mut tail: *mut Option<Box<ListNode>> = &mut new_head;
        let mut i = 0usize;
        let mut j = vals.len().saturating_sub(1);
        unsafe {
            while i <= j {
                let n = Box::new(ListNode::new(vals[i]));
                *tail = Some(n);
                tail = &mut (*tail).as_mut().unwrap().next as *mut Option<Box<ListNode>>;
                if i != j {
                    let n2 = Box::new(ListNode::new(vals[j]));
                    *tail = Some(n2);
                    tail = &mut (*tail).as_mut().unwrap().next as *mut Option<Box<ListNode>>;
                }
                i += 1;
                if j == 0 {
                    break;
                }
                j -= 1;
            }
        }
        *head = new_head;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reorder_list_iter_space_on() {
        let mut input = build_list(&[1, 2, 3, 4]);
        let output = build_list(&[1, 4, 2, 3]);
        Solution::reorder_list_iter_space_on(&mut input);
        assert!(list_equal(&input, &output));
    }
}
