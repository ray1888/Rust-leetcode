pub struct Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        if amount == 0 || coins.len() == 0 {
            return 1;
        }
        let mut dp = vec![0; (amount + 1) as usize ];
        dp[0] = 1;
        for i in 1..coins.len()+1{
            for j in 1..amount+1{
                let mut cur_coin = 0;
                if j >= coins[i-1] {
                    cur_coin = dp[(j-coins[i-1]) as usize];
                }
                dp[j as usize] = dp[j as usize] + cur_coin
            }
        }
        return dp[amount as usize];
    }
}