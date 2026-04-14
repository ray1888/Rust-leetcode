pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        let row = matrix.len();
        let col = matrix[0].len();
        let mut i = 0i32;
        let mut j = (row * col - 1) as i32;

        while i <= j {
            let mid = i + (j - i) / 2;
            let mid_col = (mid as usize) % col;
            let mid_row = (mid as usize) / col;
            if target == matrix[mid_row][mid_col] {
                return true;
            }
            if target < matrix[mid_row][mid_col] {
                j = mid - 1;
            } else {
                i = mid + 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_matrix() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert_eq!(Solution::search_matrix(matrix.clone(), 3), true);
        assert_eq!(Solution::search_matrix(matrix.clone(), 13), false);
    }
}
