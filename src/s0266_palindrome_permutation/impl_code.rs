pub struct Solution;

impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        use std::collections::HashMap;
        let mut alpha_map: HashMap<char, i32> = HashMap::new();
        for item in s.chars() {
            *alpha_map.entry(item).or_insert(0) += 1;
        }
        let mut cnt = 0;
        for value in alpha_map.values() {
            if value % 2 == 1 {
                cnt += 1;
            }
            if cnt > 1 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_permute_palindrome() {
        assert_eq!(
            Solution::can_permute_palindrome("tactcoa".to_string()),
            true
        );
        assert_eq!(Solution::can_permute_palindrome("code".to_string()), false);
    }
}
