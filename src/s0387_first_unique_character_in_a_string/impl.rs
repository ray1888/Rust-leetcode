pub struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map = [0;26];
        s.chars().for_each(|c| { map[c as usize -'a' as usize] += 1; });
        for (i, c) in s.chars().enumerate() { 
            if map[c as usize -'a' as usize] == 1 {
                return i as i32;
            }
        }
        -1
    }
}

