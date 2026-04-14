pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        if s.len() == 0 {
            return "".to_string();
        }
        s.split(' ')
            .map(|s| s.chars().rev().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words() {
        assert_eq!(
            Solution::reverse_words("Let's take LeetCode contest".to_string()),
            "s'teL ekat edoCteeL tsetnoc"
        );
    }
}
