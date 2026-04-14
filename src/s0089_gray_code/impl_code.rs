pub struct Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        if n == 0 {
            return vec![0];
        }
        let mut result: Vec<i32> = vec![0];
        let mut tmp = 1;
        for _ in 0..n {
            for j in (0..result.len()).rev() {
                result.push(tmp + result[j]);
            }
            tmp <<= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gray_code() {
        assert_eq!(Solution::gray_code(2), vec![0, 1, 3, 2]);
    }
}
