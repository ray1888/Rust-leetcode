pub struct Solution;

impl Solution {
    pub fn is_straight(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        nums.sort_unstable();
        let mut joker = 0;
        for i in 0..4 {
            if nums[i] == 0 {
                joker += 1;
                continue;
            }
            if nums[i] == nums[i + 1] {
                return false;
            }
        }
        nums[4] - nums[joker] < 5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_straight() {
        let cases = vec![
            (vec![0, 0, 1, 2, 5], true),
            (vec![1, 2, 4, 5, 6], false),
            (vec![0, 2, 4, 5, 6], true),
        ];
        for (input, expected) in cases {
            assert_eq!(Solution::is_straight(input), expected);
        }
    }
}
