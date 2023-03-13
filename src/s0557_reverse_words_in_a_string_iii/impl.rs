pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        if s.len() == 0 {
            return "".to_string();
        }
        s.split(' ')
            .map(|s| s.chars().rev().collect::<String>())
            .collect::<Vec<_>>().join(" ")

    }
}