pub struct Solution;

use std::cmp::min;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.len() == 0 {
            return 0;
        }
        let height = triangle.len();
        let max_length = triangle[height-1].len();
        let mut dp = vec![0;max_length+1];
       
        for i in (0..height).rev(){
            let level_length = triangle[i].len();
            for j in 0..level_length{
                dp[j] = min(dp[j], dp[j+1]) + triangle[i][j];
            }
        }

        return dp[0];
    }
}
