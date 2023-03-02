pub struct Solution;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        
        let mut node = head.as_mut().unwrap();
        println!("{:?}", node);
        let mut duplicate = node.val;

        while let Some(mut next) = node.next.take() {
            if next.val == duplicate {
                node.next = next.next;
            } else {
                duplicate = next.val;
                //还原现场
                node.next = Some(next);
                node = node.next.as_mut().unwrap();
            }
        }

        head

    }
}