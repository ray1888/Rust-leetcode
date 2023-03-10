pub struct Solution;

use std::cmp::Ordering;

use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0 as i32;
        let mut high = (nums.len()-1) as i32;

        while low <= high{
            let mid:i32 = low + (high-low) /2;
            if mid < 0 {
                break;
            }
            let numMid = nums[mid as usize];
            match target.cmp(&numMid){
                Ordering::Equal=> return mid as i32,
                Ordering::Less=>{},
                Ordering::Greater=>{},
            }
            if nums[mid as usize] >= nums[low as usize]{
                if target >= nums[low as usize] && target < nums[mid as usize]{
                    high = mid -1;
                } else {
                    low = mid + 1;
                }
            } else {
                if target > nums[mid as usize] && target <= nums[high as usize]{
                    low = mid + 1;
                } else {
                    high = mid - 1;
                }
            }
        } 
        return -1;
    }
}