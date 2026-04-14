pub struct Solution;

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        let mut num = 0i32;
        for ch in s.chars() {
            num = num * 26 + ((ch as u8 - b'A' + 1) as i32);
        }
        num
    }

    pub fn title_to_number_right_to_left(s: String) -> i32 {
        let mut num = 0i32;
        let mut base = 1i32;
        for ch in s.chars().rev() {
            num += ((ch as u8 - b'A' + 1) as i32) * base;
            base *= 26;
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_to_number() {
        assert_eq!(Solution::title_to_number("A".to_string()), 1);
        assert_eq!(Solution::title_to_number("AB".to_string()), 28);
    }

    #[test]
    fn test_title_to_number_right_to_left() {
        assert_eq!(Solution::title_to_number_right_to_left("A".to_string()), 1);
        assert_eq!(
            Solution::title_to_number_right_to_left("AB".to_string()),
            28
        );
    }
}
