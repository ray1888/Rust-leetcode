pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return -1;
        } else if nums.len() == 1 {
            return nums[0];
        }
        let mut i = 0usize;
        let mut j = nums.len() - 1;
        while i < j {
            let mid = i + (j - i) / 2;
            if nums[mid] > nums[j] {
                i = mid + 1;
            } else {
                j = mid;
            }
        }
        nums[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min() {
        let cases = vec![
            (vec![1], 1),
            (vec![3, 4, 5, 1, 2], 1),
            (vec![1, 2, 3, 4, 5], 1),
            (vec![2, 1], 1),
        ];
        for (nums, expected) in cases {
            assert_eq!(Solution::find_min(nums), expected);
        }
    }
}
