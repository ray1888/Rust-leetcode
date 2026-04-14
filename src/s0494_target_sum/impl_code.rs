pub struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum: i32 = nums.iter().sum();
        let target = target.abs();
        if sum < target || (sum + target) % 2 != 0 {
            return 0;
        }
        let bag_size = ((sum + target) / 2) as usize;
        let mut dp = vec![0; bag_size + 1];
        dp[0] = 1;
        for num in nums {
            let num = num as usize;
            for j in (num..=bag_size).rev() {
                dp[j] += dp[j - num];
            }
        }
        dp[bag_size]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_target_sum_ways() {
        assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
        assert_eq!(Solution::find_target_sum_ways(vec![1], 1), 1);
    }
}
