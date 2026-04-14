pub struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        if n == 0 {
            return vec![];
        }
        let n = n as usize;
        let mut matrix = vec![vec![0; n]; n];
        let mut num = 1;
        let tar = n * n;
        let mut l = 0usize;
        let mut t = 0usize;
        let mut r = n - 1;
        let mut b = n - 1;
        while num <= tar {
            for i in l..=r {
                matrix[t][i] = num as i32;
                num += 1;
            }
            t += 1;
            for i in t..=b {
                matrix[i][r] = num as i32;
                num += 1;
            }
            if r == 0 {
                break;
            }
            r -= 1;
            for i in (l..=r).rev() {
                matrix[b][i] = num as i32;
                num += 1;
            }
            if b == 0 {
                break;
            }
            b -= 1;
            for i in (t..=b).rev() {
                matrix[i][l] = num as i32;
                num += 1;
            }
            l += 1;
        }
        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_matrix() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5],]
        );
    }
}
