pub struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.is_empty() || t.is_empty() {
            return false;
        }
        let (s_bytes, t_bytes) = (s.as_bytes(), t.as_bytes());
        if s_bytes.len() != t_bytes.len() {
            return false;
        }
        let mut s2t = [0u8; 256];
        let mut t2s = [0u8; 256];
        for i in 0..s_bytes.len() {
            let x = s_bytes[i];
            let y = t_bytes[i];
            if s2t[x as usize] != 0 && s2t[x as usize] != y
                || t2s[y as usize] != 0 && t2s[y as usize] != x
            {
                return false;
            }
            s2t[x as usize] = y;
            t2s[y as usize] = x;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_isomorphic() {
        assert_eq!(
            Solution::is_isomorphic("foo".to_string(), "bar".to_string()),
            false
        );
    }
}
