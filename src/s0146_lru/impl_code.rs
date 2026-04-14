use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct ListNode {
    key: i32,
    val: i32,
    prev: Option<Rc<RefCell<ListNode>>>,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(key: i32, val: i32) -> Self {
        Self {
            key,
            val,
            prev: None,
            next: None,
        }
    }
}

#[derive(Default)]
struct List {
    head: Option<Rc<RefCell<ListNode>>>,
    tail: Option<Rc<RefCell<ListNode>>>,
}

impl List {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push_back(&mut self, node: Option<Rc<RefCell<ListNode>>>) {
        if let Some(t) = self.tail.take() {
            if let Some(n) = &node {
                t.borrow_mut().next = Some(n.clone());
                n.borrow_mut().prev = Some(t);
            }
        } else {
            self.head = node.clone();
        }

        self.tail = node;
    }

    fn pop_front(&mut self) -> Option<Rc<RefCell<ListNode>>> {
        if let Some(h) = self.head.take() {
            if let Some(n) = h.borrow_mut().next.take() {
                n.borrow_mut().prev = None;
                self.head = Some(n);
            } else {
                self.head = None;
                self.tail = None;
            }

            Some(h)
        } else {
            None
        }
    }

    fn unlink_node(
        &mut self,
        node: Option<Rc<RefCell<ListNode>>>,
    ) -> Option<Rc<RefCell<ListNode>>> {
        if let Some(n) = node {
            let prev = n.borrow_mut().prev.take();
            let next = n.borrow_mut().next.take();

            if let Some(p) = &prev {
                p.borrow_mut().next = next.clone();
            } else {
                self.head = next.clone();
            }

            if let Some(n) = &next {
                n.borrow_mut().prev = prev;
            } else {
                self.tail = prev;
            }

            Some(n)
        } else {
            None
        }
    }
}

pub struct LRUCache {
    cap: usize,
    used: usize,
    data: HashMap<i32, Rc<RefCell<ListNode>>>,
    list: List,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            cap: capacity as usize,
            used: 0,
            data: HashMap::new(),
            list: List::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.data.get(&key) {
            let val = node.borrow().val;

            let node = self.list.unlink_node(Some(node.clone()));
            self.list.push_back(node);

            val
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.data.get_mut(&key) {
            node.borrow_mut().val = value;

            let node = self.list.unlink_node(Some(node.clone()));
            self.list.push_back(node);
        } else {
            if self.used == self.cap {
                if let Some(node) = self.list.pop_front() {
                    self.data.remove(&node.borrow().key);
                    self.used -= 1;
                }
            }

            let new_node = Rc::new(RefCell::new(ListNode::new(key, value)));
            self.data.insert(key, new_node.clone());
            self.list.push_back(Some(new_node));
            self.used += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LRUCache;

    const LRU_LENGTH: i32 = 4;
    const OVER_LENGTH: i32 = 5;

    #[test]
    fn test_lru_get_unexisted() {
        let mut lru = LRUCache::new(LRU_LENGTH);
        lru.put(4, 4);
        let val = lru.get(3);
        assert_eq!(val, -1, "get unexisted failed, not matched");
    }

    #[test]
    fn test_lru_save_get() {
        let mut lru = LRUCache::new(LRU_LENGTH);
        lru.put(4, 4);
        let val = lru.get(4);
        assert_eq!(val, 4, "get puted val failed, not matched");
    }

    #[test]
    fn test_lru_save_get_with_length() {
        let mut lru = LRUCache::new(LRU_LENGTH);
        for i in 0..LRU_LENGTH {
            lru.put(i, i);
        }
        for i in 0..LRU_LENGTH {
            let val = lru.get(i);
            assert_eq!(val, i, "lru can't get puted val within length");
        }
    }

    #[test]
    fn test_lru_save_get_over_length() {
        let mut lru = LRUCache::new(LRU_LENGTH);
        for i in 0..OVER_LENGTH {
            lru.put(i, i);
        }
        for i in 0..OVER_LENGTH {
            let val = lru.get(i);
            if i == 0 {
                assert_eq!(
                    val, -1,
                    "lru can't get puted val within length, key is {i}, value is {val}"
                );
            } else {
                assert_eq!(
                    val, i,
                    "lru can't get puted val within length, key is {i}, value is {val}"
                );
            }
        }
    }

    #[test]
    fn test_leetcode_use_case() {
        let mut lru = LRUCache::new(2);
        lru.put(1, 1);
        lru.put(2, 2);
        assert_eq!(lru.get(1), 1);
        lru.put(3, 3);
        assert_eq!(lru.get(2), -1);
        lru.put(4, 4);
        assert_eq!(lru.get(1), -1);
        assert_eq!(lru.get(3), 3);
        assert_eq!(lru.get(4), 4);
    }
}
