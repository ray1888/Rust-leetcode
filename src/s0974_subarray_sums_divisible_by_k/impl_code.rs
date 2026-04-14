pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut sum = 0;
        let mut res = 0;
        map.insert(0, 1);
        for num in nums {
            sum += num;
            let modulus = ((sum % k) + k) % k;
            res += map.get(&modulus).unwrap_or(&0);
            *map.entry(modulus).or_insert(0) += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subarrays_div_by_k() {
        assert_eq!(Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5), 7);
    }
}
