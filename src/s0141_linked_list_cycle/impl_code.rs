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
    pub fn has_cycle(head: Option<Box<ListNode>>) -> bool {
        let mut fast = head.as_ref().map(|b| b.as_ref() as *const ListNode);
        let mut slow = head.as_ref().map(|b| b.as_ref() as *const ListNode);

        loop {
            if fast.is_none() {
                return false;
            }
            fast = unsafe {
                (*fast.unwrap())
                    .next
                    .as_ref()
                    .map(|b| b.as_ref() as *const ListNode)
            };
            if fast.is_none() {
                return false;
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
                return true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_cycle_empty() {
        assert_eq!(Solution::has_cycle(None), false);
    }

    #[test]
    fn test_has_cycle_single_node() {
        let n1 = Some(Box::new(ListNode::new(1)));
        assert_eq!(Solution::has_cycle(n1), false);
    }

    #[test]
    fn test_has_cycle_two_nodes_no_cycle() {
        let n2 = Some(Box::new(ListNode::new(2)));
        let n1 = Some(Box::new(ListNode { val: 1, next: n2 }));
        assert_eq!(Solution::has_cycle(n1), false);
    }

    #[test]
    fn test_has_cycle_linear_list() {
        let n3 = Some(Box::new(ListNode::new(3)));
        let n2 = Some(Box::new(ListNode { val: 2, next: n3 }));
        let n1 = Some(Box::new(ListNode { val: 1, next: n2 }));
        assert_eq!(Solution::has_cycle(n1), false);
    }
}
