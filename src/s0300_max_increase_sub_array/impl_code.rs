pub struct Solution;

use std::cmp::max;

impl Solution {
    fn bsearch(d: &Vec<i32>, arr_length: i32, lastest_num: i32) -> i32 {
        let mut low = 0;
        let mut high = arr_length - 1;
        while low <= high {
            let mid = low + (high - low) / 2;
            if lastest_num > d[mid as usize] {
                low = mid + 1;
            } else if lastest_num < d[mid as usize] {
                high = mid - 1;
            } else {
                return mid;
            }
        }
        low
    }

    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut dp: Vec<i32> = vec![0; nums.len()];
        dp[0] = 1;
        let mut max_value = 1;

        for i in 1..nums.len() {
            for j in 0..i {
                let mut cur = 1;
                if nums[j] < nums[i] {
                    cur = dp[j] + 1;
                }
                dp[i] = max(cur, dp[i]);
            }
            max_value = max(max_value, dp[i]);
        }
        max_value
    }

    pub fn length_of_lis2(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut dp: Vec<i32> = vec![0; nums.len()];
        let mut arr_length = 0;

        for num in nums {
            let index = Solution::bsearch(&dp, arr_length, num);
            dp[index as usize] = num;
            if index == arr_length {
                arr_length += 1;
            }
        }

        arr_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_long_incr_sub_seq() {
        assert_eq!(Solution::length_of_lis(vec![1, 8, 2, 6, 4, 5]), 4);
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }

    #[test]
    fn test_length_of_lis2() {
        assert_eq!(Solution::length_of_lis2(vec![1, 8, 2, 6, 4, 5]), 4);
        assert_eq!(
            Solution::length_of_lis2(vec![10, 9, 2, 5, 3, 7, 101, 18]),
            4
        );
    }
}
