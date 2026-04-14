pub struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        if n == 1 {
            return 1;
        } else if n < 1 {
            return 0;
        }
        let mut dp = vec![0; n];
        dp[0] = 1;
        let mut p2 = 0usize;
        let mut p3 = 0usize;
        let mut p5 = 0usize;
        for i in 1..n {
            let ugly_num = (dp[p2] * 2).min(dp[p3] * 3).min(dp[p5] * 5);
            dp[i] = ugly_num;
            if ugly_num == dp[p2] * 2 {
                p2 += 1;
            }
            if ugly_num == dp[p3] * 3 {
                p3 += 1;
            }
            if ugly_num == dp[p5] * 5 {
                p5 += 1;
            }
        }
        dp[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_ugly_number() {
        assert_eq!(Solution::nth_ugly_number(10), 12);
        assert_eq!(Solution::nth_ugly_number(6), 6);
    }
}
