pub struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_two() {
        let cases = vec![
            (1, true),
            (2, true),
            (4, true),
            (8, true),
            (15, false),
            (16, true),
        ];
        for (input, expected) in cases {
            assert_eq!(Solution::is_power_of_two(input), expected);
        }
    }
}
