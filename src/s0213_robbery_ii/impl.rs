pub struct Solution ;

use std::cmp::max;

use std::cmp::max;

impl Solution {
    fn rob_internal(nums: Vec<i32>) -> i32  {
        let length = nums.len();
        if length == 0 {
            return 0;
        }
        let mut result:Vec<i32> = vec!(0;length+1);
        result[1] = nums[0];
        
        for i in 1..length{
            result[i+1] =  max(result[i-1]+nums[i], result[i])
        }
        return result[length];
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() < 2{
            return nums[0];
        }
        if nums.len() == 2{
            return max(nums[0], nums[1]);
        }
        return max(
            Solution::rob_internal(nums[0..nums.len()-1].to_vec()),
            Solution::rob_internal(nums[1..nums.len()].to_vec())
        );
    }
}