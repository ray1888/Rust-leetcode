pub struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let length = nums.len();
        let mut result: i32 = nums[0] + nums[1] + nums[2];
        let mut min = (target - result).abs();
        nums.sort();
        for k in (2..length).rev() {
            let mut i = 0;
            let mut j = k - 1;
            while i < j {
                let sum = nums[i] + nums[j] + nums[k];
                if sum == target {
                    return sum;
                } else if sum < target {
                    i += 1;
                } else {
                    j -= 1;
                }
                let diff = (target - sum).abs();
                if diff < min {
                    result = sum;
                    min = diff;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum_closest() {
        assert_eq!(Solution::three_sum_closest(vec![-5, 1, 3, 4, 5], 1), 1);
        assert_eq!(Solution::three_sum_closest(vec![-3, 1, 3, 4, 5], -1), 1);
        assert_eq!(Solution::three_sum_closest(vec![0, 1, 2], 3), 3);
    }
}
