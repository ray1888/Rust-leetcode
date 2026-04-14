pub struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        let mut n = n;
        while n % 3 == 0 {
            n /= 3;
        }
        n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_three() {
        let cases = vec![(9, true), (8, false), (27, true), (0, false), (1, true)];
        for (input, expected) in cases {
            assert_eq!(Solution::is_power_of_three(input), expected);
        }
    }
}
