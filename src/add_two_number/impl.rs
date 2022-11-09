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

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut pos: i32 = 0;

        let mut p1 = l1;
        let mut p2 = l2;
        let mut p = dummy.as_mut();

        while !p1.is_none() || !p2.is_none() || pos != 0 {
            let mut sum = pos;
            if !p1.is_none() {
                sum += p1.as_ref().unwrap().val;
                p1 = p1.unwrap().next;
            }
            if !p2.is_none() {
                sum += p2.as_ref().unwrap().val;
                p2 = p2.unwrap().next;
            }
            let val = sum % 10;
            // below method of calling is both ok
            // method 1
            // p.as_mut().unwrap().next = Some(Box::new(ListNode::new(val)));
            // p = p.unwrap().next().as_mut();
            // method 2
            if let Some(cur) = p {
                cur.next = Some(Box::new(ListNode::new(val)));
                p = cur.next.as_mut();
            }
            pos = sum / 10;
        }
        dummy.unwrap().next
    }
}
