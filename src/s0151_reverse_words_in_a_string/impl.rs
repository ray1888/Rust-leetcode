pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        if s.len() == 0 {
            return "".to_string();
        }
        s.split_whitespace()
            .map(|s| s.chars().collect::<String>()).rev()
            .collect::<Vec<_>>().join(" ")
    }
}