pub struct Solution;

use std::cmp::{max, min};

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let mut max_value = 0;
        let mut i = 0usize;
        let mut j = height.len() - 1;
        while i < j {
            let cur = min(height[i], height[j]) * (j - i) as i32;
            max_value = max(max_value, cur);
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
        max_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
}
