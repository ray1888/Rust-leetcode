pub struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        if s.len() < 2 {
            return s.len() as i32;
        }
        let s = s.as_bytes();
        let mut left = 0usize;
        let mut right = 0usize;
        let mut res = 0;
        let mut max_count = 0;
        let mut freq = vec![0i32; 26];

        while right < s.len() {
            let idx = (s[right] - b'A') as usize;
            freq[idx] += 1;
            max_count = max_count.max(freq[idx]);
            right += 1;

            if right - left > max_count as usize + k as usize {
                let left_idx = (s[left] - b'A') as usize;
                freq[left_idx] -= 1;
                left += 1;
            }
            res = res.max(right - left);
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_replacement() {
        assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4);
    }
}
