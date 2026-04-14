pub struct Solution;

impl Solution {
    fn _search_insert(nums: &[i32], start: i32, end: i32, target: i32) -> i32 {
        if start < 0 || end < 0 {
            return 0;
        }
        let mid = start + (end - start) / 2;
        if mid < start {
            if nums[start as usize] < target {
                return start + 1;
            } else {
                return start;
            }
        } else if mid > end {
            if nums[end as usize] > target {
                return end;
            } else {
                return end + 1;
            }
        }
        if nums[mid as usize] > target {
            Solution::_search_insert(nums, start, mid - 1, target)
        } else if nums[mid as usize] < target {
            Solution::_search_insert(nums, mid + 1, end, target)
        } else {
            mid
        }
    }

    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        Solution::_search_insert(&nums, 0, nums.len() as i32 - 1, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert_position() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 8], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 8], 9), 4);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 8], 6), 3);
        assert_eq!(Solution::search_insert(vec![2, 3, 5, 6], 1), 0);
        assert_eq!(Solution::search_insert(vec![1], 1), 0);
    }
}
