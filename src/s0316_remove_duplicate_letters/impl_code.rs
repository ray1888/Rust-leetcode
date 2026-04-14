pub struct Solution;

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let s = s.as_bytes();
        let mut left = [0; 26];
        for ch in s {
            left[(ch - b'a') as usize] += 1;
        }

        let mut stack: Vec<u8> = Vec::new();
        let mut in_stack = [false; 26];

        for ch in s {
            let idx = (ch - b'a') as usize;
            if !in_stack[idx] {
                while let Some(&last) = stack.last() {
                    let last_idx = (last - b'a') as usize;
                    if ch < &last && left[last_idx] > 0 {
                        stack.pop();
                        in_stack[last_idx] = false;
                    } else {
                        break;
                    }
                }
                stack.push(*ch);
                in_stack[idx] = true;
            }
            left[idx] -= 1;
        }

        String::from_utf8(stack).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicate_letters() {
        assert_eq!(
            Solution::remove_duplicate_letters("bcabc".to_string()),
            "abc"
        );
        assert_eq!(
            Solution::remove_duplicate_letters("cbacdcbc".to_string()),
            "acdb"
        );
    }
}
