pub struct Solution;

use std::cmp::max;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut dp:Vec<i32> = vec!(0;nums.len());
        dp[0] = 1;
        let mut max_value = 1;
        
        for i in 1..nums.len() {
            for j in 0..i{
                let mut cur = 1;
                if nums[j] < nums[i] {
                    cur = dp[j] + 1;
                }
                dp[i] = max(cur, dp[i]);
            }
            max_value = max(max_value, dp[i]);
        }
        return max_value;
    }
}