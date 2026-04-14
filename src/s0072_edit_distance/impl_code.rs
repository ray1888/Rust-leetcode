pub struct Solution;

use std::cmp::min;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        if word1.is_empty() && word2.is_empty() {
            return 0;
        } else if word1.is_empty() {
            return word2.len() as i32;
        } else if word2.is_empty() {
            return word1.len() as i32;
        }
        let length1 = word1.len();
        let length2 = word2.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; length2 + 1]; length1 + 1];
        for i in 1..=length2 {
            dp[0][i] = i as i32;
        }
        for i in 1..=length1 {
            dp[i][0] = i as i32;
        }
        let w1: Vec<char> = word1.chars().collect();
        let w2: Vec<char> = word2.chars().collect();
        for i in 1..=length1 {
            for j in 1..=length2 {
                if w1[i - 1] == w2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = min(dp[i - 1][j], min(dp[i - 1][j - 1], dp[i][j - 1])) + 1;
                }
            }
        }
        dp[length1][length2]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_distance() {
        assert_eq!(
            Solution::min_distance("car".to_string(), "ba".to_string()),
            2
        );
        assert_eq!(Solution::min_distance("".to_string(), "".to_string()), 0);
        assert_eq!(
            Solution::min_distance("abc".to_string(), "abc".to_string()),
            0
        );
        assert_eq!(
            Solution::min_distance("abd".to_string(), "abu".to_string()),
            1
        );
    }
}
