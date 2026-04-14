pub struct Solution;

impl Solution {
    pub fn min_distance(houses: Vec<i32>, k: i32) -> i32 {
        let mut houses = houses;
        houses.sort();
        let n = houses.len();
        let k = k as usize;
        let mut med_sum = vec![vec![0; n]; n];
        for i in (0..n - 1).rev() {
            for j in i + 1..n {
                med_sum[i][j] = med_sum[i + 1][j - 1] + houses[j] - houses[i];
            }
        }

        let mut dp = vec![vec![i32::MAX; k + 1]; n];
        for i in 0..n {
            dp[i][1] = med_sum[0][i];
            for j in 2..=k.min(i + 1) {
                for i0 in 0..i {
                    if dp[i0][j - 1] != i32::MAX {
                        dp[i][j] = dp[i][j].min(dp[i0][j - 1] + med_sum[i0 + 1][i]);
                    }
                }
            }
        }
        dp[n - 1][k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_distance() {
        assert_eq!(Solution::min_distance(vec![1, 4, 8, 10, 20], 3), 5);
        assert_eq!(Solution::min_distance(vec![7, 4, 6, 1], 1), 8);
    }
}
