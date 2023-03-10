pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len()-1);
        
        while left <= right{
            let mid = left + (right-left) / 2;
            let midNum = nums[mid];
            match target.cmp(&midNum) {
                Ordering::Less=>  {
                    if mid as i32 - 1 < 0 {
                        break;
                    } else {
                        right = mid -1;
                    }
                },
                Ordering::Greater=> left = mid +1 ,
                Ordering::Equal=> return mid as i32  ,
            }
        }
        return -1
    }
}