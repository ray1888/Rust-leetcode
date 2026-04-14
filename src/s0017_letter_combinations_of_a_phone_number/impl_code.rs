pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let digit_map: std::collections::HashMap<u8, Vec<u8>> = [
            (b'2', vec![b'a', b'b', b'c']),
            (b'3', vec![b'd', b'e', b'f']),
            (b'4', vec![b'g', b'h', b'i']),
            (b'5', vec![b'j', b'k', b'l']),
            (b'6', vec![b'm', b'n', b'o']),
            (b'7', vec![b'p', b'q', b'r', b's']),
            (b'8', vec![b't', b'u', b'v']),
            (b'9', vec![b'w', b'x', b'y', b'z']),
        ]
        .iter()
        .cloned()
        .collect();

        let mut result = vec![];
        let mut elem = vec![];
        let digits = digits.into_bytes();
        Self::dfs_phone(&digits, 0, &mut elem, &digit_map, &mut result);
        result
    }

    fn dfs_phone(
        digits: &Vec<u8>,
        index: usize,
        elem: &mut Vec<u8>,
        d_map: &std::collections::HashMap<u8, Vec<u8>>,
        result: &mut Vec<String>,
    ) {
        if index == digits.len() {
            result.push(String::from_utf8(elem.clone()).unwrap());
            return;
        }
        if let Some(val) = d_map.get(&digits[index]) {
            for &item in val {
                elem.push(item);
                Self::dfs_phone(digits, index + 1, elem, d_map, result);
                elem.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_letter_combinations() {
        assert_eq!(
            Solution::letter_combinations("123".to_string()),
            Vec::<String>::new()
        );
        let mut result = Solution::letter_combinations("23".to_string());
        result.sort();
        let mut expected = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        expected.sort();
        assert_eq!(result, expected);
    }
}
