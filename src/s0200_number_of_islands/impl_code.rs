pub struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }
        let row = grid.len();
        let col = grid[0].len();
        let mut visits = vec![vec![false; col]; row];
        let mut number = 0;

        for i in 0..row {
            for j in 0..col {
                if grid[i][j] == '1' && !visits[i][j] {
                    number += 1;
                    Self::dfs_island(&grid, &mut visits, i, j);
                }
            }
        }
        number
    }

    fn dfs_island(grid: &Vec<Vec<char>>, visits: &mut Vec<Vec<bool>>, i: usize, j: usize) {
        let row = grid.len();
        let col = grid[0].len();
        if i >= row || j >= col || grid[i][j] == '0' || visits[i][j] {
            return;
        }
        visits[i][j] = true;
        if i + 1 < row {
            Self::dfs_island(grid, visits, i + 1, j);
        }
        if i > 0 {
            Self::dfs_island(grid, visits, i - 1, j);
        }
        if j + 1 < col {
            Self::dfs_island(grid, visits, i, j + 1);
        }
        if j > 0 {
            Self::dfs_island(grid, visits, i, j - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_num_islands() {
        let cases = vec![
            (
                vec![
                    vec!['1', '1', '1', '1', '0'],
                    vec!['1', '1', '0', '1', '0'],
                    vec!['1', '1', '0', '0', '0'],
                    vec!['0', '0', '0', '0', '0'],
                ],
                1,
            ),
            (
                vec![
                    vec!['1', '1', '0', '0', '0'],
                    vec!['1', '1', '0', '0', '0'],
                    vec!['0', '0', '1', '0', '0'],
                    vec!['0', '0', '0', '1', '1'],
                ],
                3,
            ),
        ];

        for (grid, expected) in cases {
            let actual = Solution::num_islands(grid);
            assert_eq!(actual, expected);
        }
    }
}
