pub struct Solution;

use std::cmp::max;

impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let sum: i32 = stones.iter().sum();
        let bag_size = (sum / 2) as usize;
        let mut dp = vec![0; bag_size + 1];
        for i in 1..=stones.len() {
            let stone = stones[i - 1] as usize;
            for j in (stone..=bag_size).rev() {
                dp[j] = max(dp[j], dp[j - stone] + stone);
            }
        }
        sum - 2 * dp[bag_size] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_last_stone_weight_ii() {
        assert_eq!(Solution::last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]), 1);
    }
}
