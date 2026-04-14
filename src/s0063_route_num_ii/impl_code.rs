pub struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid.len() == 0 {
            return 0;
        }
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();

        let mut dp: Vec<Vec<i32>> = vec![vec![0; n as usize]; m as usize];

        for i in 0..n {
            if obstacle_grid[0][i] == 0 {
                dp[0][i as usize] = 1;
            } else {
                break;
            }
        }

        for i in 0..m {
            if obstacle_grid[i][0] == 0 {
                dp[i as usize][0] = 1;
            } else {
                break;
            }
        }

        for i in 1..m {
            for j in 1..n {
                if obstacle_grid[i as usize][j as usize] == 1 {
                    continue;
                }
                dp[i as usize][j as usize] =
                    dp[i as usize][(j - 1) as usize] + dp[(i - 1) as usize][j as usize];
            }
        }

        return dp[(m - 1) as usize][(n - 1) as usize];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_paths_with_obstacles() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0],
            ]),
            2
        );
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![1, 0],]), 0);
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 0], vec![1, 1], vec![0, 0],]),
            0
        );
    }
}
