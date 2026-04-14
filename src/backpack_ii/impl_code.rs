pub struct Solution;

impl Solution {
    pub fn back_pack_ii(m: i32, a: &[i32], v: &[i32]) -> i32 {
        let n = a.len();
        let m = m as usize;
        let mut dp = vec![vec![0; m + 1]; n + 1];
        let mut max_value = 0;

        for i in 1..=n {
            for j in 1..=m {
                dp[i][j] = dp[i - 1][j];
                if j as i32 >= a[i - 1] {
                    dp[i][j] = dp[i][j].max(dp[i - 1][j - a[i - 1] as usize] + v[i - 1]);
                }
                max_value = max_value.max(dp[i][j]);
            }
        }
        max_value as i32
    }

    pub fn back_pack_ii_on_space(m: i32, a: &[i32], v: &[i32]) -> i32 {
        let m = m as usize;
        let mut dp = vec![0; m + 1];

        for i in 1..=a.len() {
            for j in (a[i - 1] as usize..=m).rev() {
                dp[j] = dp[j].max(dp[j - a[i - 1] as usize] + v[i - 1]);
            }
        }
        dp[m] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_back_pack_ii() {
        assert_eq!(
            Solution::back_pack_ii(10, &[2, 3, 5, 7], &[1, 5, 2, 4]),
            9
        );
        assert_eq!(
            Solution::back_pack_ii(10, &[2, 3, 8], &[2, 5, 8]),
            10
        );
    }

    #[test]
    fn test_back_pack_ii_on_space() {
        assert_eq!(
            Solution::back_pack_ii_on_space(10, &[2, 3, 5, 7], &[1, 5, 2, 4]),
            9
        );
        assert_eq!(
            Solution::back_pack_ii_on_space(10, &[2, 3, 8], &[2, 5, 8]),
            10
        );
    }
}
