pub struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut result = Vec::new();
        let mut max_val = 0;

        let mut i = 0usize;
        let mut j = k - 1;
        while j < nums.len() {
            if i == 0 || max_val == nums[i - 1] {
                max_val = nums[i];
                for t in (i + 1..=j).rev() {
                    if max_val < nums[t] {
                        max_val = nums[t];
                    }
                }
            } else {
                if nums[j] > max_val {
                    max_val = nums[j];
                }
            }
            result.push(max_val);
            i += 1;
            j += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sliding_window() {
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
    }
}
