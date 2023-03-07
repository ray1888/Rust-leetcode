pub struct Solution;

use std::cmp::min;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m<0 || n < 0 {
            return 0;
        }

        let mut dp :Vec<Vec<i32>> = vec![vec![0;n as usize];m as usize];

        for i in 0..n{
            dp[0][i as usize] = 1;
        }

        for i in 0..m{
            dp[i as usize][0] = 1;
        }

        for i in 1..m{
            for j in 1..n{
                dp[i as usize][j as usize] = dp[i as usize][(j-1) as usize] + dp[(i-1) as usize][j as usize];
            }
        }

        return dp[(m-1) as usize][(n-1) as usize]
    }
}