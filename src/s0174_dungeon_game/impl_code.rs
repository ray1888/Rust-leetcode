pub struct Solution;

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        if dungeon.is_empty() || dungeon[0].is_empty() {
            return 0;
        }
        let row = dungeon.len();
        let col = dungeon[0].len();
        let mut dp = vec![vec![i32::MAX; col + 1]; row + 1];
        dp[row][col - 1] = 1;
        dp[row - 1][col] = 1;
        for i in (0..row).rev() {
            for j in (0..col).rev() {
                let minn = dp[i + 1][j].min(dp[i][j + 1]);
                dp[i][j] = (minn - dungeon[i][j]).max(1);
            }
        }
        dp[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_calculate_minimum_hp() {
        assert_eq!(
            Solution::calculate_minimum_hp(vec![
                vec![-2, -3, 3],
                vec![-5, -10, 1],
                vec![10, 30, -5],
            ]),
            7
        );
    }
}
