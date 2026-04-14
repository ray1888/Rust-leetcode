pub struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let n = digits.len();
        let mut p = n as i32 - 1;
        while p >= 0 && digits[p as usize] == 9 {
            digits[p as usize] = 0;
            p -= 1;
        }
        if p == -1 {
            let mut result = vec![0; n + 1];
            result[0] = 1;
            result
        } else {
            digits[p as usize] += 1;
            digits
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        let cases = vec![
            (vec![1, 2, 3], vec![1, 2, 4]),
            (vec![1, 2, 9, 9], vec![1, 3, 0, 0]),
            (vec![1, 2, 9], vec![1, 3, 0]),
            (vec![1, 9, 9, 5], vec![1, 9, 9, 6]),
            (vec![9, 9, 9], vec![1, 0, 0, 0]),
        ];
        for (input, expected) in cases {
            assert_eq!(Solution::plus_one(input), expected);
        }
    }
}
