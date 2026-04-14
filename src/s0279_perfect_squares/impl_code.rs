pub struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        for i in 1..=n {
            dp[i] = i as i32;
            let mut j = 1;
            while j * j <= i {
                dp[i] = dp[i].min(dp[i - j * j] + 1);
                j += 1;
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_squares() {
        assert_eq!(Solution::num_squares(1), 1);
        assert_eq!(Solution::num_squares(13), 2);
    }
}
