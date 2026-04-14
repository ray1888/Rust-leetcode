pub struct Solution;

impl Solution {
    fn is_equal(s: u8, p: u8) -> bool {
        s == p || p == b'.'
    }

    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let m = s.len();
        let n = p.len();
        let mut dp = vec![vec![false; n + 1]; m + 1];

        dp[0][0] = true;

        for j in 1..=n {
            if p[j - 1] == b'*' {
                dp[0][j] = dp[0][j - 2];
            }
        }

        for i in 1..=m {
            for j in 1..=n {
                let sc = s[i - 1];
                let pc = p[j - 1];
                if Self::is_equal(sc, pc) {
                    dp[i][j] = dp[i - 1][j - 1];
                } else if pc == b'*' {
                    let pre_char = p[j - 2];
                    if Self::is_equal(sc, pre_char) {
                        dp[i][j] = dp[i][j - 2] || dp[i][j - 1] || dp[i - 1][j];
                    } else {
                        dp[i][j] = dp[i][j - 2];
                    }
                }
            }
        }

        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match() {
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
        assert_eq!(Solution::is_match("".to_string(), ".*".to_string()), true);
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
    }
}
