pub struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        for i in 0..nums.len() - 1 {
            if nums[i] > nums[i + 1] {
                return i as i32;
            }
        }
        (nums.len() - 1) as i32
    }

    pub fn find_peak_element_binary_search(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut low = 0i32;
        let mut high = (nums.len() - 1) as i32;
        while low < high {
            let mid = low + (high - low) / 2;
            let left = if mid - 1 >= 0 {
                nums[(mid - 1) as usize]
            } else {
                i32::MIN
            };
            let right = if ((mid + 1) as usize) < nums.len() {
                nums[(mid + 1) as usize]
            } else {
                i32::MAX
            };
            if nums[mid as usize] > left && nums[mid as usize] > right {
                return mid;
            } else if left > nums[mid as usize] {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        low
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_peak_element() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 2]), 2);
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 2, 4, 2]), 2);
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 4]), 3);
    }

    #[test]
    fn test_find_peak_element_binary_search() {
        assert_eq!(
            Solution::find_peak_element_binary_search(vec![1, 2, 3, 2]),
            2
        );
        assert_eq!(
            Solution::find_peak_element_binary_search(vec![1, 2, 3, 2, 4, 2]),
            2
        );
        assert_eq!(
            Solution::find_peak_element_binary_search(vec![1, 2, 3, 4]),
            3
        );
    }
}
