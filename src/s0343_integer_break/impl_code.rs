pub struct Solution;

use std::cmp::max;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        for i in 2..=n {
            let mut cur_max = 0;
            for j in 1..i {
                cur_max = max(cur_max, max(j * (i - j), j * dp[i - j]));
            }
            dp[i] = cur_max;
        }
        dp[n] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_integer_break() {
        assert_eq!(Solution::integer_break(10), 36);
    }
}
