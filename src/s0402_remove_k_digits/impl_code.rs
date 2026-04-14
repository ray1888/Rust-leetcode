pub struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut k = k as usize;
        let mut stack: Vec<u8> = Vec::new();
        for digit in num.bytes() {
            while k > 0 && !stack.is_empty() && digit < *stack.last().unwrap() {
                stack.pop();
                k -= 1;
            }
            stack.push(digit);
        }
        stack.truncate(stack.len() - k);
        let ans = String::from_utf8(stack).unwrap();
        let ans = ans.trim_start_matches('0');
        if ans.is_empty() {
            "0".to_string()
        } else {
            ans.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_kdigits() {
        let cases = vec![
            (("1432219".to_string(), 3), "1219".to_string()),
            (("10200".to_string(), 1), "200".to_string()),
            (("123".to_string(), 3), "0".to_string()),
        ];
        for ((num, k), expected) in cases {
            assert_eq!(Solution::remove_kdigits(num, k), expected);
        }
    }
}
