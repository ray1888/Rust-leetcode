pub struct Solution;

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let length = nums.len();
        if length == 0 {
            return 0;
        }
        let mut count = vec![1; length];
        let mut dp = vec![0; length];
        let mut max_length = 0;
        for j in 0..length {
            for i in 0..j {
                if nums[j] > nums[i] {
                    if dp[i] >= dp[j] {
                        dp[j] = dp[i] + 1;
                        count[j] = count[i];
                    } else if dp[i] + 1 == dp[j] {
                        count[j] += count[i];
                    }
                }
            }
            max_length = max_length.max(dp[j]);
        }
        let mut sum = 0;
        for i in 0..length {
            if dp[i] == max_length {
                sum += count[i];
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_number_of_lis() {
        assert_eq!(Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]), 2);
    }
}
