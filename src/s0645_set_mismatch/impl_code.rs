pub struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut xor = 0;
        let mut xor0 = 0;
        let mut xor1 = 0;

        for (index, item) in nums.iter().enumerate() {
            xor ^= item ^ (index as i32 + 1);
        }

        let right_most_bit = xor & (-xor);
        for item in &nums {
            if (item & right_most_bit) != 0 {
                xor1 ^= item;
            } else {
                xor0 ^= item;
            }
        }
        for index in 0..nums.len() {
            let i = index as i32 + 1;
            if (i & right_most_bit) != 0 {
                xor1 ^= i;
            } else {
                xor0 ^= i;
            }
        }

        for item in &nums {
            if *item == xor0 {
                return vec![xor0, xor1];
            }
        }
        vec![xor1, xor0]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_error_nums() {
        let cases = vec![(vec![1, 2, 2, 4], vec![2, 3])];

        for (nums, expected) in cases {
            let actual = Solution::find_error_nums(nums);
            assert_eq!(actual, expected);
        }
    }
}
