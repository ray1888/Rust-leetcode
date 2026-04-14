pub struct Solution;

use std::cmp::min;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if coins.is_empty() {
            return 0;
        }
        let amount = amount as usize;
        let length = coins.len();
        let mut dp = vec![vec![0; amount + 1]; length + 1];
        for j in 0..=amount {
            dp[0][j] = i32::MAX;
        }
        for i in 1..=length {
            for j in 1..=amount {
                let coin = coins[i - 1] as usize;
                let used = if j >= coin {
                    let prev = dp[i][j - coin];
                    if prev == i32::MAX {
                        i32::MAX
                    } else {
                        prev + 1
                    }
                } else {
                    i32::MAX
                };
                dp[i][j] = min(dp[i - 1][j], used);
            }
        }
        if dp[length][amount] == i32::MAX {
            -1
        } else {
            dp[length][amount]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_change() {
        assert_eq!(Solution::coin_change(vec![1, 2, 4], 4), 1);
        assert_eq!(Solution::coin_change(vec![1, 2], 4), 2);
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
    }
}
