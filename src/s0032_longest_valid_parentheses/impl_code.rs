pub struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut max_ans = 0;
        let mut stack: Vec<i32> = vec![-1];
        let bytes = s.as_bytes();
        for i in 0..bytes.len() {
            if bytes[i] == b'(' {
                stack.push(i as i32);
            } else {
                stack.pop();
                if stack.is_empty() {
                    stack.push(i as i32);
                } else {
                    let top = *stack.last().unwrap();
                    max_ans = max_ans.max(i as i32 - top);
                }
            }
        }
        max_ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_valid_parentheses() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
    }
}
