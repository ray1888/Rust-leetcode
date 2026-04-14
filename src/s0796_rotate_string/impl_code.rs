pub struct Solution;

impl Solution {
    pub fn rotate_string(a: String, b: String) -> bool {
        if a.len() != b.len() {
            return false;
        }
        if a.is_empty() {
            return true;
        }
        let aa = format!("{}{}", a, a);
        aa.contains(&b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_string() {
        assert_eq!(
            Solution::rotate_string("abcde".to_string(), "cdeab".to_string()),
            true
        );
        assert_eq!(
            Solution::rotate_string("abcde".to_string(), "cedab".to_string()),
            false
        );
        assert_eq!(
            Solution::rotate_string("abcde".to_string(), "ceab".to_string()),
            false
        );
    }
}
