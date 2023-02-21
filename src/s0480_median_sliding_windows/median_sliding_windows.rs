

pub struct Finder {
    heap_max: BinaryHeap<Reverse<i32>>,
    heap_min: BinaryHeap<i32>,
    delay_map: HashMap<i32,i32>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            heap_max: BinaryHeap::new(),
            heap_min: BinaryHeap::new(),
            delay_map: HashMap::new(),
        }
    }

    fn make_balanced(&mut self) {
        if self.heap_max.len() + 1 < self.heap_min.len(){
            self.heap_max.push(Reverse(self.heap_min.pop()));
            // 调整最小堆
            self.prune_heap_min();
        } else if self.heap_min.len( ) < self.heap_max.len(){
            self.heap_min.push(self.heap_max.pop().unwrap().0);
            // 调整最大堆
            self.prune_heap_max();
        }
    }

    fn prune_heap_min(&mut self) {
        while self.heap_min.len() >  0 {
            let num = self.heap_min.peak().unwrap();
            match self.delay_map.get(&num){
                Some(i) => {
                    if i > 1 {
                        _  = self.delay_map.insert(num, i-1);                        
                    } else {
                        self.delay_map.remove(&num);
                    }
                    self.heap_min.pop();
                },
                None => break,
            }
        }
    }

    fn prune_heap_max(&mut self){
        while self.heap_max.len() >  0 {
            let num = self.heap_max.peak().unwrap().0;
            match self.delay_map.get(&num){
                Some(i) => {
                    if i > 1 {
                        _  = self.delay_map.insert(num, i-1);                        
                    } else {
                        self.delay_map.remove(&num);
                    }
                    self.heap_max.pop();
                },
                None => break,
            }
        }
    }

    fn insert(&mut self, val: i32) {
        if f.heap_min.len() == 0 || val <= f.heap_min.peak(){
            f.heap_min.push(val);
        } else {
            f.heap_max.push(Revserse(val));
        }
        self.make_balanced();
    }

    fn erase(&mut self, val:i32) {
        match self.delay_map.get(&w) {
            Some(count) => { self.delay_map.insert(w, count + 1); }
            None => { map.insert(w, 1); }
        }
        if val <= self.heap_min.peak() {

        } else {

        }

    }

}

pub struct Solution;
use std::collections::HashMap;
use std::cmp::Reverse;

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        if nums.len() == 0 {
            return Vec::new();
        }
        let mut f = Finder::new();
        let mut delayMap :HashMap<i32, i32> = HashMap::new();

    }
}