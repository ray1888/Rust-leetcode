pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() < 2 {
            return false;
        }
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, -1);
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            let mod_val = if k == 0 { sum } else { sum % k };
            if let Some(&j) = map.get(&mod_val) {
                if (i as i32) - j > 1 {
                    return true;
                }
            } else {
                map.insert(mod_val, i as i32);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_check_subarray_sum() {
        assert_eq!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6), true);
    }
}
