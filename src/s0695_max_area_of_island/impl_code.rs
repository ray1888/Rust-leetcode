pub struct Solution;

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 0 {
                    continue;
                }
                let area = Self::area_of_island(&mut grid, i as i32, j as i32);
                if area > res {
                    res = area;
                }
            }
        }
        res
    }

    fn is_in_grid(grid: &Vec<Vec<i32>>, x: i32, y: i32) -> bool {
        x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[0].len() as i32
    }

    fn area_of_island(grid: &mut Vec<Vec<i32>>, x: i32, y: i32) -> i32 {
        if !Self::is_in_grid(grid, x, y) || grid[x as usize][y as usize] == 0 {
            return 0;
        }
        grid[x as usize][y as usize] = 0;
        let mut total = 1;
        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        for (dx, dy) in dirs.iter() {
            total += Self::area_of_island(grid, x + dx, y + dy);
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_area_of_island() {
        let grid = vec![
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 1],
            vec![0, 0, 0, 1, 1],
        ];
        assert_eq!(Solution::max_area_of_island(grid), 4);
    }
}
