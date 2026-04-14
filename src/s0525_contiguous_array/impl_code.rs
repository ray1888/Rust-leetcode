pub struct Solution;

use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, -1);
        let mut max_len = 0;
        let mut count = 0;
        for (i, num) in nums.iter().enumerate() {
            if *num == 1 {
                count += 1;
            } else {
                count -= 1;
            }
            if let Some(&val) = map.get(&count) {
                max_len = max(max_len, i as i32 - val);
            } else {
                map.insert(count, i as i32);
            }
        }
        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_length() {
        assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
    }
}
