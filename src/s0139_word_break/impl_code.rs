pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        if s.is_empty() {
            return false;
        }
        let word_set: HashSet<String> = word_dict.into_iter().collect();
        let n = s.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true;
        for i in 1..=n {
            for j in (0..i).rev() {
                if dp[j] && word_set.contains(&s[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_break() {
        assert_eq!(
            Solution::word_break(
                "leetcode".to_string(),
                vec!["leet".to_string(), "code".to_string()]
            ),
            true
        );
        assert_eq!(
            Solution::word_break(
                "applepenapple".to_string(),
                vec!["apple".to_string(), "pen".to_string()]
            ),
            true
        );
        assert_eq!(
            Solution::word_break(
                "catsandog".to_string(),
                vec![
                    "cats".to_string(),
                    "dog".to_string(),
                    "sand".to_string(),
                    "and".to_string(),
                    "cat".to_string()
                ]
            ),
            false
        );
    }
}
