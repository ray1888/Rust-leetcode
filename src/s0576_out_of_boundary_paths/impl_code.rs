pub struct Solution;

impl Solution {
    pub fn find_paths(m: i32, n: i32, n_move: i32, start_row: i32, start_column: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let n_move = n_move as usize;
        let start_row = start_row as usize;
        let start_column = start_column as usize;
        if n_move == 0 {
            return 0;
        }
        let modulo = 1_000_000_007i64;
        let direct: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut dp = vec![vec![0i64; n]; m];
        for _ in 0..n_move {
            let mut tmp = vec![vec![0i64; n]; m];
            for x in 0..m {
                for y in 0..n {
                    for &(dx, dy) in &direct {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                            tmp[x][y] += 1;
                        } else {
                            tmp[x][y] = (tmp[x][y] + dp[nx as usize][ny as usize]) % modulo;
                        }
                    }
                }
            }
            dp = tmp;
        }
        dp[start_row][start_column] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_paths() {
        assert_eq!(Solution::find_paths(2, 2, 2, 0, 0), 6);
        assert_eq!(Solution::find_paths(1, 3, 3, 0, 1), 12);
    }
}
