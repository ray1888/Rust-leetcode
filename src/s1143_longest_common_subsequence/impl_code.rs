pub struct Solution;

use std::cmp::max;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let l1 = text1.len();
        let l2 = text2.len();
        if l1 == 0 || l2 == 0 {
            return 0;
        }
        let t1: Vec<char> = text1.chars().collect();
        let t2: Vec<char> = text2.chars().collect();
        let mut dp = vec![vec![0; l2 + 1]; l1 + 1];
        for i in 1..=l1 {
            for j in 1..=l2 {
                if t1[i - 1] == t2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }
        dp[l1][l2]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_longest_common_subsequence() {
        assert_eq!(
            Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()),
            3
        );
    }
}
