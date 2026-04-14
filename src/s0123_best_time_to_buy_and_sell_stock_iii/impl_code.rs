pub struct Solution;

use std::cmp::max;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let k = 2usize;
        if prices.is_empty() {
            return 0;
        }
        let n = prices.len();
        let mut dp = vec![vec![vec![0; 2]; k + 1]; n];
        for i in 0..n {
            for j in 0..=k {
                dp[i][j][0] = 0;
                dp[i][j][1] = i32::MIN;
            }
        }
        for i in 0..n {
            for t in (1..=k).rev() {
                if i == 0 {
                    dp[i][t][0] = 0;
                    dp[i][t][1] = -prices[i];
                    continue;
                }
                dp[i][t][0] = max(dp[i - 1][t][0], dp[i - 1][t][1] + prices[i]);
                dp[i][t][1] = max(dp[i - 1][t][1], dp[i - 1][t - 1][0] - prices[i]);
            }
        }
        dp[n - 1][k][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
