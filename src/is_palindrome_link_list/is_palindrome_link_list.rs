// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
pub struct Solution;

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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return true;
        }

        // 快慢指针切分链表
        let (mut fast, mut slow) = (&head, &head);
        while fast.is_some() {
            fast = fast
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .map(|x| &x.next)
                .unwrap_or(&None);
            slow = slow.as_ref().map(|x| &x.next).unwrap();
        }
        let s = slow as *const Option<Box<ListNode>> as *mut Option<Box<ListNode>>;
        let slow = unsafe { (*s).take() };

        // 反转后半部分链表
        let new_head = revert_link(None, slow);

        // 比较是否回文
        is_palindrome_help(head, new_head)
    }
}

pub fn is_palindrome_help(head1: Option<Box<ListNode>>, head2: Option<Box<ListNode>>) -> bool {
    match (head1, head2) {
        (None, None) | (None, Some(_)) | (Some(_), None) => true,
        (Some(h1), Some(h2)) => {
            if h1.val == h2.val {
                is_palindrome_help(h1.next, h2.next)
            } else {
                false
            }
        }
    }
}

pub fn revert_link(
    pre: Option<Box<ListNode>>,
    mut curr: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if curr.is_none() {
        return pre;
    }
    use std::borrow::BorrowMut;
    let next = std::mem::replace(curr.as_mut().unwrap().next.borrow_mut(), pre);
    revert_link(curr, next)
}
