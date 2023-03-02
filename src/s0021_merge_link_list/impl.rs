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
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        // 因为这里dummy只是给了p一个可变借用，因此最后的时候，dummy.next是可以调用的，所有权没有迁移
        let mut p = &mut dummy;
        while l1.is_some() && l2.is_some() {
            let (p1, p2) = (l1.as_deref_mut().unwrap(), l2.as_deref_mut().unwrap());
            if p1.val < p2.val {
                // Option.Take需要用&mut self ，所以l1拿出来的必须是mut，所以导致l1也要变成mut
                let next = p1.next.take();
                p.next = l1.take();
                l1 = next;
            } else {
                // Option.Take需要用&mut self 
                let next = p2.next.take();
                p.next = l2.take();
                l2 = next;
            }
            p = p.next.as_deref_mut().unwrap();
        }
        p.next = l1.or(l2);
        dummy.next
    }
}
