pub struct Solution;

use std::cmp::max;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut dp:Vec<i32> = vec!(0;nums.len());
        dp[0] = 1;
        let mut max_value = 1;
        
        for i in 1..nums.len() {
            for j in 0..i{
                let mut cur = 1;
                if nums[j] < nums[i] {
                    cur = dp[j] + 1;
                }
                dp[i] = max(cur, dp[i]);
            }
            max_value = max(max_value, dp[i]);
        }
        return max_value;
    }
}

impl Solution {
    fn bsearch(d: &Vec<i32>, arrLength: i32, lastestNum: i32) -> i32 {
        let mut low = 0;
        let mut high = arrLength -1 ;
        while low <= high {
            let mid = low + (high-low) / 2;
            if lastestNum > d[mid as usize] {
                low = mid+1;
            } else if lastestNum < d[mid as usize] {
                high = mid-1;
            } else {
               return mid;
            }
        }
        return low; 
    }

    pub fn length_of_lis2(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut dp :Vec<i32> = vec!(0;nums.len());
        let mut arr_length = 0;

        for i in 0..nums.len(){
            let index = Solution::bsearch(&dp, arr_length, nums[i]);
            println!("i={:?}, index={:?}",i,index);
            dp[index as usize] = nums[i];
            if index == arr_length{
                arr_length +=1;
            }
        }

        return arr_length;
    }
}