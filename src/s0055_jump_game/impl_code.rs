pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.is_empty() {
            return true;
        }
        Self::can_jump_rec(0, &nums)
    }

    fn can_jump_rec(start: usize, nums: &Vec<i32>) -> bool {
        if start >= nums.len() - 1 {
            return true;
        }
        let max_jump = (nums[start] as usize) + start;
        let jump = if max_jump < nums.len() - 1 {
            max_jump
        } else {
            nums.len() - 1
        };
        for i in (start + 1..=jump).rev() {
            if Self::can_jump_rec(i, nums) {
                return true;
            }
        }
        false
    }

    pub fn can_jump_dp(nums: Vec<i32>) -> bool {
        let mut max_length = 0usize;
        for i in 0..nums.len() {
            if max_length >= nums.len() - 1 {
                return true;
            }
            if i > max_length {
                return false;
            }
            max_length = max_length.max(i + nums[i] as usize);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_can_jump() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
        assert_eq!(Solution::can_jump(vec![1, 1, 1, 0]), true);
    }

    #[test]
    fn test_can_jump_dp() {
        assert_eq!(Solution::can_jump_dp(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(Solution::can_jump_dp(vec![3, 2, 1, 0, 4]), false);
        assert_eq!(Solution::can_jump_dp(vec![1, 1, 1, 0]), true);
    }
}
