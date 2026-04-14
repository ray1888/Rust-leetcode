pub struct Solution;

impl Solution {
    pub fn is_power_of_k(n: i32, k: i32) -> bool {
        if n == 0 {
            return false;
        }
        let mut n = n;
        while n % k == 0 {
            n /= k;
        }
        n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_k() {
        let cases = vec![
            ((4, 2), true),
            ((5, 2), false),
            ((9, 3), true),
            ((8, 3), false),
            ((16, 4), true),
            ((5, 4), false),
        ];
        for ((n, k), expected) in cases {
            assert_eq!(Solution::is_power_of_k(n, k), expected);
        }
    }
}
