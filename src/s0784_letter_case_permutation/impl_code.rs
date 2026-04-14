pub struct Solution;

impl Solution {
    fn is_alpha(c: u8) -> bool {
        (b'a' <= c && c <= b'z') || (b'A' <= c && c <= b'Z')
    }

    fn dfs(s: &mut Vec<u8>, index: usize, result: &mut Vec<String>) {
        if index == s.len() {
            result.push(String::from_utf8(s.clone()).unwrap());
            return;
        }
        Self::dfs(s, index + 1, result);
        if Self::is_alpha(s[index]) {
            s[index] ^= 32;
            Self::dfs(s, index + 1, result);
            s[index] ^= 32;
        }
    }

    pub fn letter_case_permutation(s: String) -> Vec<String> {
        if s.is_empty() {
            return vec![];
        }
        let mut result = vec![];
        let mut bytes = s.into_bytes();
        Self::dfs(&mut bytes, 0, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_letter_case_permutation() {
        let mut actual = Solution::letter_case_permutation("a1b2".to_string());
        actual.sort();
        let mut expected = vec!["a1b2", "a1B2", "A1b2", "A1B2"];
        expected.sort();
        assert_eq!(actual, expected);
    }
}
