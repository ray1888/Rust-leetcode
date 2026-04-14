pub struct Solution;

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let length = nums.len();
        let mut i = 0usize;
        let mut j = length - 1;
        while i <= j {
            let mut mid = i + (j - i) / 2;
            if mid > length - 1 {
                break;
            }
            if mid > 0 && nums[mid] == nums[mid - 1] {
                mid -= 1;
            } else if mid + 1 < length && nums[mid] == nums[mid + 1] {
                // already aligned to left of pair
            } else {
                return nums[mid];
            }
            if (mid as i32 - i as i32) % 2 == 1 {
                if mid == 0 {
                    break;
                }
                j = mid - 1;
            } else {
                i = mid + 2;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_non_duplicate() {
        let cases = vec![
            (vec![2, 2, 3, 3, 6, 6, 8], 8),
            (vec![2, 2, 3, 4, 4], 3),
            (vec![5, 5, 7, 7, 8, 9, 9], 8),
        ];
        for (nums, expected) in cases {
            assert_eq!(Solution::single_non_duplicate(nums), expected);
        }
    }
}
