pub struct Solution;

use std::cmp::Ordering;

// impl Solution {
//     pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
//         if nums.len() == 0 {
//             return vec![-1,-1];
//         }
//         let leftIndex = Solution::get_index(&nums,target,true);
//         let rightIndex = Solution::get_index(&nums,target,false);
//         return vec![leftIndex,rightIndex];
//     }

//     fn get_index(nums: &Vec<i32>, target: i32,is_left:bool) ->i32 {
//         let mut pos:i32 = -1;
//         let mut low = 0;
//         let mut high = nums.len()-1;
//         while low <= high {
//             let mid = low + (high -low) / 2;
//             if mid < 0 || mid >= nums.len(){
//                 break;
//             }
//             if nums[mid] == target {
//                 pos = mid as i32;
//                 if is_left{
//                     high = mid -1;
//                 } else {
//                     low = mid + 1;
//                 }
//             } else if  nums[mid] > target {
//                 low = mid + 1;
//             } else {
//                 high = mid-1;
//             }
//         }
//         return pos;
//     }
// }


impl Solution {
    pub fn search(nums: &Vec<i32>, target: i32, flag: bool) -> usize {
        let mut lt = 0_i32;
        let mut rt = nums.len() as i32 - 1;
        let mut ans = nums.len();
        while lt <= rt {
            let mid = (lt + (rt - lt) / 2) as usize;
            if (flag &&  nums[mid] >= target) || nums[mid] > target {
                rt = (mid - 1) as i32;
                ans = mid;
            } else {
                lt = (mid + 1) as i32;
            }
        }
        ans
    }
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let lt = Self::search(&nums, target, true);
        let rt = Self::search(&nums, target, false) - 1;
        if (lt <= rt && rt < nums.len() && nums[lt] == target && nums[rt] == target) {
            return [lt as i32, rt as i32].to_vec();
        } else {
            return [-1, -1].to_vec();
        }
    }
}
