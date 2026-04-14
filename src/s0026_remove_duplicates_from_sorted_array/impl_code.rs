pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut p = 1;
        let length = nums.len();
        for q in 1..length {
            if nums[q] != nums[q - 1] {
                nums[p] = nums[q];
                p += 1;
            }
        }
        p as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let cases = vec![
            (vec![1, 2, 3, 4, 4, 5], 5),
            (vec![56, 56, 59, 60, 77], 4),
            (vec![53, 54, 55, 56, 56], 4),
            (vec![53, 53, 54, 55, 56, 56], 4),
            (vec![53, 53, 54, 55, 55, 56, 56], 4),
        ];
        for (mut input, expected) in cases {
            assert_eq!(Solution::remove_duplicates(&mut input), expected);
        }
    }
}
