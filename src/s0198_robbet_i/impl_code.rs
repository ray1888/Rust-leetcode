pub struct Solution;

use std::cmp::max;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let length = nums.len();
        if length == 0 {
            return 0;
        }
        let mut result: Vec<i32> = vec![0; length + 1];
        result[1] = nums[0];
        for i in 1..length {
            result[i + 1] = max(result[i - 1] + nums[i], result[i]);
        }
        result[length]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_rob() {
        assert_eq!(Solution::rob(vec![8, 1, 9, 3]), 17);
        assert_eq!(Solution::rob(vec![8, 1, -9, 3]), 11);
        assert_eq!(Solution::rob(vec![8, 1, 9, 3, 5]), 22);
    }
}
