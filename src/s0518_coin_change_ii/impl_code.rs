pub struct Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        if amount == 0 || coins.is_empty() {
            return 1;
        }
        let amount = amount as usize;
        let mut dp = vec![0; amount + 1];
        dp[0] = 1;
        for i in 1..=coins.len() {
            for j in 1..=amount {
                let coin = coins[i - 1] as usize;
                let cur = if j >= coin { dp[j - coin] } else { 0 };
                dp[j] += cur;
            }
        }
        dp[amount]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change() {
        assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
        assert_eq!(Solution::change(4, vec![1, 2, 4]), 4);
    }
}
