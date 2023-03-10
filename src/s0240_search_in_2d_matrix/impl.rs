pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (mut x, mut y) = (0, matrix[0].len() - 1);
        while x < matrix.len() && y < matrix[0].len() {
            match matrix[x][y].cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Greater => y -= 1,
                Ordering::Less => x += 1,
            }
        }
        false
    }
}