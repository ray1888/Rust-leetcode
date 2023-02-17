use std::collections::BinaryHeap;
use std::fmt;
use std::cmp::Ordering;

#[derive(Copy,Clone, Debug, Eq)]
 struct Pair {
        right:i32,
        height:i32,
 }

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.right == other.right && self.height == other.height
    }
}


impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.right.cmp(&other.right) == Ordering::Equal{
            self.height.cmp(&other.height)
        } else {
            self.right.cmp(&other.right)
        }
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}




impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if buildings.len() == 0 {
            return vec![vec![]];
        }
        let mut result :Vec<Vec<i32>> = vec![vec![]];
        let mut pairs :Vec<Pair> = Vec::new();
        // 对buildings 进行自定义排序
        for (index, item) in buildings.iter().enumerate(){
            pairs.push(Pair{right:item[0], height:-item[2]});
            pairs.push(Pair{right:item[1], height:item[2]});
        }
        // 创建Heap 及辅助对象
        let mut heap = BinaryHeap::new();
        heap.push(0);
         let mut a =  BinaryHeap::new();

        let mut preHeight = 0;
        // 进行扫描
        for (i, item) in pairs.iter().enumerate(){
            if item.height < 0 {
                heap.push(-item.height);
            } else {
                Solution::remote_element_from_heap(heap, item);
            }
            if heap.peek() != Some(&preHeight){
                let largest = heap.pop();
                result.push(vec![heap.peek().unwrap(), largest]);
                preHeight = largest;
            }
        }
        return result 
    }
}