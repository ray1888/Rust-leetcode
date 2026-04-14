use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut prefix_sum = vec![0; n + 1];
        let mut map: HashMap<i32, usize> = HashMap::new();
        map.insert(0, 0);
        for i in 1..=n {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i - 1];
            map.entry(prefix_sum[i]).or_insert(i);
        }
        let mut max_value = 0;
        for i in (0..=n).rev() {
            let target = prefix_sum[i] - k;
            if let Some(&val) = map.get(&target) {
                max_value = max_value.max(i - val);
            }
        }
        max_value as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sub_array_len() {
        assert_eq!(Solution::max_sub_array_len(vec![1, -1, 5, -2, 3], 3), 4);
    }
}
