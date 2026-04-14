pub struct Solution;

use std::cmp::{max, min};

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return 0;
        }
        let mut max_steps = vec![0; nums.len()];
        let mut max_length = 0usize;
        for i in 0..nums.len() {
            if max_length >= nums.len() - 1 {
                return max_steps[nums.len() - 1] as i32;
            }
            if i > max_length {
                return -1;
            }
            max_length = max(max_length, i + nums[i] as usize);
            let last = min(nums.len() - 1, max_length);
            for j in ((i + 1)..=last).rev() {
                if max_steps[j] == 0 {
                    max_steps[j] = max_steps[i] + 1;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_jump() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }
}
