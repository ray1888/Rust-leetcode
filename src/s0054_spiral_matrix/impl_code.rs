pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() {
            return vec![];
        }
        let mut left = 0i32;
        let mut right = (matrix[0].len() as i32) - 1;
        let mut top = 0i32;
        let mut bottom = (matrix.len() as i32) - 1;
        let mut result = vec![];
        loop {
            for i in left..=right {
                result.push(matrix[top as usize][i as usize]);
            }
            top += 1;
            if top > bottom {
                break;
            }
            for i in top..=bottom {
                result.push(matrix[i as usize][right as usize]);
            }
            right -= 1;
            if left > right {
                break;
            }
            for i in (left..=right).rev() {
                result.push(matrix[bottom as usize][i as usize]);
            }
            bottom -= 1;
            if top > bottom {
                break;
            }
            for i in (top..=bottom).rev() {
                result.push(matrix[i as usize][left as usize]);
            }
            left += 1;
            if left > right {
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_order() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            Solution::spiral_order(matrix),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );

        let matrix2 = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        assert_eq!(
            Solution::spiral_order(matrix2),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }
}
