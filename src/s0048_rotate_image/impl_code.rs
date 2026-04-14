pub struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n / 2 {
            let j = n - 1 - i;
            let tmp = matrix[i].clone();
            matrix[i] = matrix[j].clone();
            matrix[j] = tmp;
        }
        for i in 0..n {
            for j in 0..i {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, expected);
    }
}
