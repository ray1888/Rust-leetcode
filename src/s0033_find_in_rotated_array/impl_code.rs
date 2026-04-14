pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let mut low = 0i32;
        let mut high = (nums.len() - 1) as i32;

        while low <= high {
            let mid = low + (high - low) / 2;
            if mid < 0 {
                break;
            }
            let num_mid = nums[mid as usize];
            match target.cmp(&num_mid) {
                Ordering::Equal => return mid as i32,
                _ => {}
            }
            if nums[mid as usize] >= nums[low as usize] {
                if target >= nums[low as usize] && target < nums[mid as usize] {
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            } else {
                if target > nums[mid as usize] && target <= nums[high as usize] {
                    low = mid + 1;
                } else {
                    high = mid - 1;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_rotated() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }
}
