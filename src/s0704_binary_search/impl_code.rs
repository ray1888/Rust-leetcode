pub struct Solution;

impl Solution {
    fn _search(nums: &[i32], start: i32, end: i32, target: i32) -> i32 {
        let mid = start + (end - start) / 2;
        if mid < start || mid > end {
            return -1;
        }
        if nums[mid as usize] > target {
            Solution::_search(nums, start, mid - 1, target)
        } else if nums[mid as usize] < target {
            Solution::_search(nums, mid + 1, end, target)
        } else {
            mid
        }
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        Solution::_search(&nums, 0, nums.len() as i32 - 1, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        assert_eq!(Solution::search(vec![1, 3, 5, 8], 2), -1);
        assert_eq!(Solution::search(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search(vec![1, 3, 5, 8], 9), -1);
        assert_eq!(Solution::search(vec![1, 3, 5, 8], 8), 3);
        assert_eq!(Solution::search(vec![2, 3, 5, 6], 2), 0);
        assert_eq!(Solution::search(vec![1], 1), 0);
    }
}
