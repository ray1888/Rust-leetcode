pub struct Solution;

use std::cmp::min;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        if s.len()  == 0 {
            return 0;
        }
        let length = s.len();
        let mut cut = vec![0;length+1];
        let mut dp = vec![vec![false; s.len()]; s.len()];
        
        // 构建DP
        for i in (0..length).rev(){
            for j in (i..length){
                if i == j {
                    dp[i][j] = true;
                } else if i+1 == j {
                    dp[i][j] = s.chars().nth(i).unwrap() == s.chars().nth(j).unwrap() 
                } else {
                    dp[i][j] = s.chars().nth(i).unwrap() == s.chars().nth(j).unwrap() && dp[i+1][j-1]
                }
            }
        }
        
        // 执行切割，使用第i位切割的时候，
        cut[0] = -1;
        for j in 0..length{
            cut[j+1] = cut[j] + 1;
            for i in (0..j).rev(){
                if dp[i][j] {
                    cut[j+1] = min(cut[j+1], cut[i] + 1);
                }
            }
        }

        return cut[length];
    }
}