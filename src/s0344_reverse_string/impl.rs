pub struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let len = s.len();
        let mut c;
        for i in 0..len/2 {
            c = s[i];
            s[i] = s[len-i-1];
            s[len-i-1] = c;
        }
    }
}