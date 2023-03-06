pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn main() {
    let head = Some(Box::new(ListNode::new(1)));
    let node = head.as_ref().unwrap();
    println!("{:?}", node);
    let node2 = head.as_deref().unwrap();
    println!("{:?}", node2);
}