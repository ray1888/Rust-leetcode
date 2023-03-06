pub struct Solution;

use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut current = std::i32::MIN;
        let mut max_value = std::i32::MIN;
        
        for i in 0..nums.len(){
            if current < 0  {
                current = nums[i] 
            } else {
                current += nums[i]
            }
            max_value = max(max_value, current)
        }
        return max_value 
    }
}