pub struct Solution;

impl Solution {
    pub fn back_pack_iv(m: i32, a: &[i32], v: &[i32], times: &[i32]) -> i32 {
        let n = a.len();
        let m = m as usize;
        let mut dp = vec![vec![0; m + 1]; n + 1];

        for i in 1..=n {
            for j in 1..=m {
                dp[i][j] = dp[i - 1][j];
                let mut k = m / a[i - 1] as usize;
                if times[i - 1] != 0 {
                    k = k.min(times[i - 1] as usize);
                }
                for l in 1..=k {
                    if j >= a[i - 1] as usize * l {
                        dp[i][j] = dp[i][j].max(dp[i - 1][j - a[i - 1] as usize * l] + v[i - 1] * l as i32);
                    }
                }
            }
        }
        dp[n][m] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_back_pack_iv() {
        assert_eq!(
            Solution::back_pack_iv(10, &[2, 3, 5, 7], &[1, 5, 2, 4], &[0, 0, 0, 0]),
            15
        );
        assert_eq!(
            Solution::back_pack_iv(10, &[2, 3, 5, 7], &[1, 5, 2, 4], &[0, 2, 0, 0]),
            12
        );
    }
}
