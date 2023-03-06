use std::cmp::{max,min};

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut current_max = nums[0];
        let mut current_min = nums[0];
        let mut max_value = nums[0];
        
        for i in 1..nums.len(){
            let tmp_max = current_max * nums[i];
            let tmp_min = current_min * nums[i];
            current_max = max(tmp_max, max(tmp_min, nums[i]));
            current_min = min(tmp_max, min(tmp_min, nums[i]));
            max_value = max(max_value, max(current_max, current_min));
        }
        return max_value 
    }
}