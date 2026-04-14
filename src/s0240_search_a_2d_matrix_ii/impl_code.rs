pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        let row = matrix.len();
        let mut i = 0;
        let mut j = matrix[0].len() - 1;
        while i < row && j < matrix[0].len() {
            if matrix[i][j] > target {
                if j == 0 {
                    break;
                }
                j -= 1;
            } else if matrix[i][j] < target {
                i += 1;
            } else {
                return true;
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
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        assert_eq!(Solution::search_matrix(matrix.clone(), 5), true);
        assert_eq!(Solution::search_matrix(matrix.clone(), 20), false);
    }
}
