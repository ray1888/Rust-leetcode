pub struct Solution;

impl Solution {
    pub fn largest_sum_of_averages(a: Vec<i32>, k: i32) -> f64 {
        let n = a.len();
        if n == 0 {
            return 0.0;
        }
        let k = k as usize;
        let mut p = vec![0; n + 1];
        for i in 0..n {
            p[i + 1] = a[i] + p[i];
        }
        let mut dp = vec![0.0; n];
        for i in 0..n {
            dp[i] = (p[n] - p[i]) as f64 / (n - i) as f64;
        }
        for _ in 1..k {
            for i in 0..n {
                for j in (i + 1)..n {
                    let avg = (p[j] - p[i]) as f64 / (j - i) as f64;
                    dp[i] = dp[i].max(dp[j] + avg);
                }
            }
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_largest_sum_of_averages() {
        assert_eq!(
            Solution::largest_sum_of_averages(vec![9, 1, 2, 3, 9], 3),
            20.0
        );
    }
}
