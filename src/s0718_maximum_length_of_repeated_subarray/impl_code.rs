pub struct Solution;

impl Solution {
    pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
        if a.is_empty() || b.is_empty() {
            return 0;
        }
        let la = a.len();
        let lb = b.len();
        let mut dp = vec![vec![0; lb + 1]; la + 1];
        let mut max_value = 0;
        for i in 1..=la {
            for j in 1..=lb {
                if a[i - 1] == b[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                }
                max_value = max_value.max(dp[i][j]);
            }
        }
        max_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_length() {
        assert_eq!(
            Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7]),
            3
        );
        assert_eq!(
            Solution::find_length(vec![0, 1, 1, 1, 1], vec![1, 0, 1, 0, 1]),
            2
        );
    }
}
