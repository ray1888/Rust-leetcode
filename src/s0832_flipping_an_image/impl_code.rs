pub struct Solution;

impl Solution {
    pub fn flip_and_invert_image(mut a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for row in a.iter_mut() {
            let mut i = 0;
            let mut j = row.len().saturating_sub(1);
            while i < j {
                if row[i] == row[j] {
                    row[i] ^= 1;
                    row[j] ^= 1;
                }
                i += 1;
                j -= 1;
            }
            if i == j {
                row[i] ^= 1;
            }
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_and_invert_image() {
        assert_eq!(
            Solution::flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0],]),
            vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1],]
        );
    }
}
