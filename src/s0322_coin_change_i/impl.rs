pub struct Solution;


impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount + 1; amount as usize + 1];
        dp[0] = 0;
        for coin in coins {
            (coin as usize..=amount as usize).for_each(|i| {
                dp[i] = dp[i].min(dp[i - coin as usize] + 1);
            });
        }
        if dp[amount as usize] == amount + 1 {
            -1
        } else {
            dp[amount as usize]
        }
    }
}

impl Solution {
    pub fn coin_change2(coins: Vec<i32>, amount: i32) -> i32 {
        if coins.len() == 0  {
            return 1;
        }
        if amount == 0 {
            return 0;
        }
        let length = coins.len();
        let mut dp :Vec<Vec<i32>> = vec![vec![0;(amount + 1) as usize];(length + 1) as usize];

        for i in 0..amount+1{
            dp[0][i as usize] = std::i32::MAX;
        }

        for i in 1..length+1{
            for j in 1..amount+1{
                let mut usedCoin = 0; 
                if j < coins[i-1]{
                   usedCoin  = std::i32::MAX;
                } else {
                    usedCoin = dp[i as usize ][(j-coins[i-1]) as usize ];
                }
                if usedCoin != std::i32::MAX{
                    usedCoin += 1
                }
                dp[i as usize][j as usize] = min(dp[(i-1) as usize][j as usize], usedCoin);
            }  
        }

        if dp[length as usize ][amount as usize] == std::i32::MAX  {
            return -1 
        }
        return dp[length as usize][amount as usize];
    }
}


