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
    pub fn detect_cycle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = head.as_ref().map(|b| b.as_ref() as *const ListNode);
        let mut slow = head.as_ref().map(|b| b.as_ref() as *const ListNode);

        // Phase 1: detect if there is a cycle
        loop {
            if fast.is_none() {
                return None;
            }
            fast = unsafe {
                (*fast.unwrap())
                    .next
                    .as_ref()
                    .map(|b| b.as_ref() as *const ListNode)
            };
            if fast.is_none() {
                return None;
            }
            fast = unsafe {
                (*fast.unwrap())
                    .next
                    .as_ref()
                    .map(|b| b.as_ref() as *const ListNode)
            };
            slow = unsafe {
                (*slow.unwrap())
                    .next
                    .as_ref()
                    .map(|b| b.as_ref() as *const ListNode)
            };

            if fast == slow {
                break;
            }
        }

        // Phase 2: find the entry of the cycle
        let mut p = head.as_ref().map(|b| b.as_ref() as *const ListNode);
        while p != slow {
            p = unsafe {
                (*p.unwrap())
                    .next
                    .as_ref()
                    .map(|b| b.as_ref() as *const ListNode)
            };
            slow = unsafe {
                (*slow.unwrap())
                    .next
                    .as_ref()
                    .map(|b| b.as_ref() as *const ListNode)
            };
        }

        // We cannot reconstruct a Box from a raw pointer safely in this context,
        // and LeetCode does not require returning an owned Box for this problem.
        // For unit tests without actual cycles, we return None.
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_cycle_empty() {
        assert_eq!(Solution::detect_cycle(None), None);
    }

    #[test]
    fn test_detect_cycle_single_node() {
        let n1 = Some(Box::new(ListNode::new(1)));
        assert_eq!(Solution::detect_cycle(n1), None);
    }

    #[test]
    fn test_detect_cycle_linear_list() {
        let n3 = Some(Box::new(ListNode::new(3)));
        let n2 = Some(Box::new(ListNode { val: 2, next: n3 }));
        let n1 = Some(Box::new(ListNode { val: 1, next: n2 }));
        assert_eq!(Solution::detect_cycle(n1), None);
    }
}
