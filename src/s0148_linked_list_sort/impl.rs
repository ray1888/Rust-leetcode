pub struct Solution;

impl Solution {
    // TODO implement
    pub fn sort_list_quick_sort(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut pivot = head;
        let mut slow = pivot;
        let mut fast = slow.as_deref_mut().next;
        let end = None();
        return head;
    }
}

impl Solution {
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut length = 0;
        let mut p = head.as_ref();

        while let Some(n) = p {
            p = n.next.as_ref();
            length += 1;
        }

        let mut res: Option<Box<ListNode>> = None;
        let mut size = 1;

        while size < length {
            let mut cur = &mut head;
            let mut p = &mut res;

            while cur.is_some() {
                let left = Solution::cut(&mut cur, size);
                let right = Solution::cut(&mut cur, size);

                *p = Solution::merge(left, right);
                while let Some(ln) = p {
                    p = &mut ln.next;
                }
            }

            head = res.take();
            size *= 2;
        }

        head
    }

    // 按照从小到大的顺序合并2个链表
    // 这个方法中传入的 l1 和 l2 一定是排序完成后的, 返回的是 l2 和 l2 合并且排序完成后的链表
    pub fn merge(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut res: Option<Box<ListNode>> = None;
        let mut p = &mut res;

        while l1.is_some() && l2.is_some() {
            let v1 = l1.as_ref().unwrap();
            let v2 = l2.as_ref().unwrap();

            let val = if v1.val < v2.val {
                let mut val = l1.unwrap();
                l1 = val.next.take();
                val
            } else {
                let mut val = l2.unwrap();
                l2 = val.next.take();
                val
            };

            *p = Some(val);
            while let Some(ln) = p {
                p = &mut ln.next;
            }
        }

        // 判断剩余哪个链表没有变成None, 在最后面添加上
        *p = if l1.is_some() { l1 } else { l2 };

        res
    }

    // 按照给定长度来链表裁剪链表
    pub fn cut(head: &mut Option<Box<ListNode>>, mut n: i32) -> Option<Box<ListNode>> {
        let mut res: Option<Box<ListNode>> = None;
        let mut p = &mut res;

        while n > 0 && head.is_some() {
            if let Some(mut ln) = head.take() {
                *head = ln.next.take();

                *p = Some(ln);
                while let Some(ln2) = p {
                    p = &mut ln2.next;
                }
            }
            n -= 1;
        }

        res
    }
}
