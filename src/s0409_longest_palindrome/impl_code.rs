pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![false; n]; n];
        let mut max_len = 0;
        for i in (0..n).rev() {
            for j in i..n {
                if i == j {
                    dp[i][j] = true;
                } else if i + 1 == j {
                    dp[i][j] = s[i] == s[j];
                } else {
                    dp[i][j] = s[i] == s[j] && dp[i + 1][j - 1];
                }
                if dp[i][j] {
                    max_len = max_len.max(j - i + 1);
                }
            }
        }
        max_len as i32
    }

    pub fn longest_palindrome_expand(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let s = s.as_bytes();
        let mut max_len = 0;
        for i in 0..s.len() {
            let len1 = Self::expand(s, i as i32, i as i32);
            let len2 = Self::expand(s, i as i32, i as i32 + 1);
            max_len = max_len.max(len1.max(len2));
        }
        max_len
    }

    fn expand(s: &[u8], mut left: i32, mut right: i32) -> i32 {
        let mut length = 0;
        while left >= 0 && right < s.len() as i32 && s[left as usize] == s[right as usize] {
            length = right - left + 1;
            left -= 1;
            right += 1;
        }
        length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(Solution::longest_palindrome("aba".to_string()), 3);
        assert_eq!(Solution::longest_palindrome("abba".to_string()), 4);
        assert_eq!(Solution::longest_palindrome("".to_string()), 0);
        assert_eq!(Solution::longest_palindrome("abcga".to_string()), 1);
    }

    #[test]
    fn test_longest_palindrome_expand() {
        assert_eq!(Solution::longest_palindrome_expand("aba".to_string()), 3);
        assert_eq!(Solution::longest_palindrome_expand("abba".to_string()), 4);
        assert_eq!(Solution::longest_palindrome_expand("".to_string()), 0);
        assert_eq!(Solution::longest_palindrome_expand("abcga".to_string()), 1);
    }
}
