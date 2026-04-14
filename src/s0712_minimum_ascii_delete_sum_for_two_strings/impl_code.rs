pub struct Solution;

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        if s1.is_empty() || s2.is_empty() {
            return 0;
        }
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let l1 = s1.len();
        let l2 = s2.len();
        let mut dp = vec![vec![0; l2 + 1]; l1 + 1];
        for i in (0..l1).rev() {
            dp[i][l2] = dp[i + 1][l2] + s1[i] as i32;
        }
        for i in (0..l2).rev() {
            dp[l1][i] = dp[l1][i + 1] + s2[i] as i32;
        }
        for i in (0..l1).rev() {
            for j in (0..l2).rev() {
                if s1[i] == s2[j] {
                    dp[i][j] = dp[i + 1][j + 1];
                } else {
                    dp[i][j] = (dp[i + 1][j] + s1[i] as i32).min(dp[i][j + 1] + s2[j] as i32);
                }
            }
        }
        dp[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_delete_sum() {
        assert_eq!(
            Solution::minimum_delete_sum("sea".to_string(), "eat".to_string()),
            231
        );
    }
}
