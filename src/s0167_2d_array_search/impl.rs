pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut low = 0;
        let mut high = numbers.len() -1;
        while low < high {
            let sum = numbers[low] + numbers[high];
            match sum.cmp(&target){
                Ordering::Equal => return vec![(low+1) as i32,(high+1) as i32],
                Ordering::Greater => high -=1,
                Ordering::Less => low += 1,
            }
        }
        return vec![];
    }
}