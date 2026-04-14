pub struct Solution;

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut t = vec![vec![0; n]; m];
        for i in 0..n {
            for j in 0..m {
                t[j][i] = matrix[i][j];
            }
        }
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        assert_eq!(
            Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9],]),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9],]
        );
        assert_eq!(
            Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6],]),
            vec![vec![1, 4], vec![2, 5], vec![3, 6],]
        );
    }
}
