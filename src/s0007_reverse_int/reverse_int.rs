
pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let mut mx = x;
        let mut result :i32 = 0;
        while mx != 0 {
            if result < std::i32::MIN / 10 || result > std::i32::MAX / 10 {
                return 0
            }
            result = result * 10 + mx%10;
            mx = mx/10;
        }
        return result;
    }
}