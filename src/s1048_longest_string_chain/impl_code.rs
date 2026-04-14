use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut words = words;
        words.sort_by_key(|w| w.len());
        let mut dp: HashMap<String, i32> = HashMap::new();
        let mut max_value = 1;
        for word in words {
            dp.entry(word.clone()).or_insert(1);
            let bytes = word.as_bytes();
            for i in 0..bytes.len() {
                let new_word =
                    String::from_utf8_lossy(&[&bytes[..i], &bytes[i + 1..]].concat()).to_string();
                if let Some(&val) = dp.get(&new_word) {
                    let entry = dp.get_mut(&word).unwrap();
                    *entry = (*entry).max(val + 1);
                }
            }
            let entry = dp.get(&word).unwrap();
            max_value = max_value.max(*entry);
        }
        max_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_str_chain() {
        assert_eq!(
            Solution::longest_str_chain(vec![
                "a".to_string(),
                "b".to_string(),
                "ba".to_string(),
                "bca".to_string(),
                "bda".to_string(),
                "bdca".to_string(),
            ]),
            4
        );
        assert_eq!(
            Solution::longest_str_chain(vec![
                "xbc".to_string(),
                "pcxbcf".to_string(),
                "xb".to_string(),
                "cxbc".to_string(),
                "pcxbc".to_string(),
            ]),
            5
        );
    }
}
