pub struct Solution;

use std::cmp::min;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        if word1.len() == 0 && word2.len() == 0 {
            return 0;
        } else if word1.len() == 0 {
            return word2.len() as i32;
        } else if word2.len() == 0 {
            return word1.len() as i32;
        }

        let length1 = word1.len();
        let length2 = word2.len();

        let mut dp :Vec<Vec<i32>> = vec![vec![0;(length2+1)as usize];(length1 + 1)as usize];
        for i in 1..length2+1 {
            dp[0][i as usize] = i as i32 ;
        }

        for i in 1..length1+1 {
            dp[i as usize][0] = i as i32;
        }

        // let mut word1_chars = ;
        // let mut word2_chars = ;

        for i in 1..length1+1{
            for j in 1..length2+1 {
                let c1 = word1.chars().nth(i-1).unwrap();
                let c2 = word2.chars().nth(j-1).unwrap();
                if c1 == c2  {
                    dp[i][j] = dp[(i-1) as usize][(j-1) as usize];
                } else {
                    dp[i][j] = min(dp[(i-1) as usize][(j) as usize],
                    min(dp[(i-1) as usize][(j-1) as usize], dp[(i) as usize][(j-1) as usize])) + 1;
                }
            }
        }

        dp[length1 as usize][length2 as usize]
    }
}