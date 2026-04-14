pub struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let l1 = s1.len();
        let l2 = s2.len();
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        let mut dp = vec![vec![false; l1 + 1]; l2 + 1];
        dp[0][0] = true;
        for i in 1..=l1 {
            dp[0][i] = dp[0][i - 1] && s1[i - 1] == s3[i - 1];
        }
        for j in 1..=l2 {
            dp[j][0] = dp[j - 1][0] && s2[j - 1] == s3[j - 1];
        }
        for i in 1..=l2 {
            for j in 1..=l1 {
                let k = i + j;
                dp[i][j] = (dp[i - 1][j] && s2[i - 1] == s3[k - 1])
                    || (dp[i][j - 1] && s1[j - 1] == s3[k - 1]);
            }
        }
        dp[l2][l1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_interleave() {
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_string(),
                "dbbca".to_string(),
                "aadbbcbcac".to_string()
            ),
            true
        );
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_string(),
                "dbbca".to_string(),
                "aadbbbaccc".to_string()
            ),
            false
        );
    }
}
