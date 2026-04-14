pub struct Solution;

use std::cmp::min;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.len() == 0 {
            return 0;
        }
        let height = triangle.len();
        let max_length = triangle[height - 1].len();
        let mut dp = vec![0; max_length + 1];

        for i in (0..height).rev() {
            let level_length = triangle[i].len();
            for j in 0..level_length {
                dp[j] = min(dp[j], dp[j + 1]) + triangle[i][j];
            }
        }

        return dp[0];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_total() {
        assert_eq!(
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3],]),
            11
        );
        assert_eq!(
            Solution::minimum_total(vec![vec![-1], vec![2, 3], vec![1, -1, -3],]),
            -1
        );
    }
}
