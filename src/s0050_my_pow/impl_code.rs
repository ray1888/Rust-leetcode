pub struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if x == 0.0 {
            return 0.0;
        } else if n == 0 || x == 1.0 {
            return 1.0;
        } else if x == -1.0 {
            let k = n.abs() as u32;
            if k % 2 == 0 {
                return 1.0;
            } else {
                return -1.0;
            }
        }
        let mut result: f64 = x;
        let k = n.abs() as i64;
        for _ in 1..k {
            result *= x;
        }
        if n < 0 {
            result = 1.0 / result;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_pow() {
        let cases = vec![
            (2.0, 0, 1.0),
            (2.0, 5, 32.0),
            (2.0, -5, 1.0 / 32.0),
            (-2.0, 5, -32.0),
            (-2.0, 4, 16.0),
            (-2.0, -5, -(1.0 / 32.0)),
            (1.0, -2147483648, 1.0),
            (-1.0, 2147483647, -1.0),
        ];
        for (x, n, expected) in cases {
            assert_eq!(
                Solution::my_pow(x, n),
                expected,
                "my_pow({}, {}) failed",
                x,
                n
            );
        }
    }
}
