pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.is_empty() || t.is_empty() {
            return String::new();
        }
        let s_bytes = s.as_bytes();
        let mut t_freq = [0i32; 256];
        let mut s_freq = [0i32; 256];
        for &b in t.as_bytes() {
            t_freq[b as usize] += 1;
        }
        let (mut left, mut right) = (0usize, usize::MAX);
        let (mut final_left, mut final_right) = (usize::MAX, usize::MAX);
        let mut min_w = s.len() + 1;
        let mut count = 0;
        while left < s_bytes.len() {
            if right.wrapping_add(1) < s_bytes.len() && count < t.len() {
                let r = right.wrapping_add(1);
                let b = s_bytes[r] as usize;
                s_freq[b] += 1;
                if s_freq[b] <= t_freq[b] {
                    count += 1;
                }
                right = r;
            } else {
                if right != usize::MAX && right - left + 1 < min_w && count == t.len() {
                    min_w = right - left + 1;
                    final_left = left;
                    final_right = right;
                }
                let b = s_bytes[left] as usize;
                if s_freq[b] == t_freq[b] {
                    count -= 1;
                }
                s_freq[b] -= 1;
                left += 1;
            }
        }
        if final_left != usize::MAX {
            String::from_utf8(s_bytes[final_left..=final_right].to_vec()).unwrap()
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_window() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC"
        );
        assert_eq!(
            Solution::min_window("cabwefgewcwaefgcf".to_string(), "cae".to_string()),
            "cwae"
        );
    }
}
