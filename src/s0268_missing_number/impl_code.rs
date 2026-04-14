pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let length = nums.len() as i32;
        let mut result = length;
        for (i, num) in nums.iter().enumerate() {
            result ^= i as i32 ^ num;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_missing_number() {
        let cases = vec![(vec![3, 0, 1], 2), (vec![2, 0, 1, 4], 3)];

        for (nums, expected) in cases {
            let actual = Solution::missing_number(nums);
            assert_eq!(actual, expected);
        }
    }
}
