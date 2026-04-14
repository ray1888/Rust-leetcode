pub struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let nums = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let alpha = vec![
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];

        let mut num = num;
        let mut result = String::new();

        for i in 0..nums.len() {
            while nums[i] <= num {
                num -= nums[i];
                result.push_str(alpha[i]);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_roman() {
        let tests = vec![(4, "IV"), (3, "III"), (58, "LVIII"), (1994, "MCMXCIV")];

        for (num, expected) in tests {
            assert_eq!(Solution::int_to_roman(num), expected.to_string());
        }
    }
}
