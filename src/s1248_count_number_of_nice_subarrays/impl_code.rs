pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut res = 0;
        let mut pre = vec![0; nums.len() + 1];
        let mut cnt = vec![0; nums.len() + 1];
        cnt[0] = 1;
        for i in 1..=nums.len() {
            pre[i] = pre[i - 1] + (nums[i - 1] & 1);
            if pre[i] >= k {
                res += cnt[(pre[i] - k) as usize];
            }
            cnt[pre[i] as usize] += 1;
        }
        res
    }

    pub fn number_of_subarrays_by_map(nums: Vec<i32>, k: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut pre_sum_map: HashMap<i32, i32> = HashMap::new();
        pre_sum_map.insert(0, 1);
        let mut res = 0;
        let mut pre = vec![0; nums.len() + 1];
        for i in 1..=nums.len() {
            pre[i] = pre[i - 1] + (nums[i - 1] & 1);
            if pre[i] >= k {
                if let Some(&val) = pre_sum_map.get(&(pre[i] - k)) {
                    res += val;
                }
            }
            *pre_sum_map.entry(pre[i]).or_insert(0) += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_number_of_subarrays() {
        assert_eq!(Solution::number_of_subarrays(vec![1, 1, 2, 1, 1], 3), 2);
        assert_eq!(
            Solution::number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2),
            16
        );
    }

    #[test]
    fn test_number_of_subarrays_by_map() {
        assert_eq!(
            Solution::number_of_subarrays_by_map(vec![1, 1, 2, 1, 1], 3),
            2
        );
        assert_eq!(
            Solution::number_of_subarrays_by_map(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2),
            16
        );
    }
}
