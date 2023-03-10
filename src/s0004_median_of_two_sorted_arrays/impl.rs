pub struct Solution;

use std::cmp::min;

impl Solution {

    fn find_smallest_in_sorted_arrays(nums1: &Vec<i32>, nums2: &Vec<i32>,mut k:i32) -> i32 {
        let mut length1  = nums1.len() as i32;
        let mut length2 = nums2.len() as i32;
        // 初始化两个数组的工作指针
        let mut base1 = 0;
        let mut base2 = 0;
        loop{
            if length1 == 0  {
                return nums2[(base2+k-1) as usize];
            }
            if length2 == 0  {
                return nums1[(base1+k-1) as usize];
            }
            if k == 1{
                return min(nums1[base1 as usize], nums2[base2 as usize]);
            }
            // 获取第一个数组的k/2或者第一个数组的末端数（防止溢出）
            // 同时获取k个数在上面获取玩剩余的数字
            let mut i = min(k/2, length1);
            let mut j = min(k-i, length2);

            let  a = nums1[(base1+i-1) as usize];
            let  b = nums2[(base2+j-1) as usize];
            // 如果刚刚好，两个数加起来等于K，并且两个数都一样的时候，返回哪一个都一样
            if i + j == k && a == b {
                return a;
            }

            // 否则，移动较小数字的那个坐标，并且把对应数组长度减少（因为已经那些长度不影响对第k个最小数的判断了）
            if a >= b {
                base2 += j;
                length2 -=j ;
                k-=j;
            }

            if b >= a {
                base1 += i;
                length1 -=i;
                k-=i;
            }

        }
        return -1;
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total_length = nums1.len() + nums2.len();
        if total_length % 2 == 0 {
            let a = Solution::find_smallest_in_sorted_arrays(&nums1,&nums2, (total_length/2 + 1) as i32 );
            let b =  Solution::find_smallest_in_sorted_arrays(&nums1,&nums2, (total_length/2) as i32 );
            return (a + b) as f64 / 2.0;
        } else {
            return Solution::find_smallest_in_sorted_arrays(&nums1,&nums2, (total_length/2 + 1) as i32) as f64
        }
    }
}