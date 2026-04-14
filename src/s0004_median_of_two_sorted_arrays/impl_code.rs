pub struct Solution;

use std::cmp::min;

impl Solution {
    fn find_smallest_in_sorted_arrays(nums1: &Vec<i32>, nums2: &Vec<i32>, mut k: i32) -> i32 {
        let mut length1 = nums1.len() as i32;
        let mut length2 = nums2.len() as i32;
        let mut base1 = 0;
        let mut base2 = 0;
        loop {
            if length1 == 0 {
                return nums2[(base2 + k - 1) as usize];
            }
            if length2 == 0 {
                return nums1[(base1 + k - 1) as usize];
            }
            if k == 1 {
                return min(nums1[base1 as usize], nums2[base2 as usize]);
            }
            let i = min(k / 2, length1);
            let j = min(k - i, length2);

            let a = nums1[(base1 + i - 1) as usize];
            let b = nums2[(base2 + j - 1) as usize];
            if i + j == k && a == b {
                return a;
            }

            if a >= b {
                base2 += j;
                length2 -= j;
                k -= j;
            }

            if b >= a {
                base1 += i;
                length1 -= i;
                k -= i;
            }
        }
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total_length = nums1.len() + nums2.len();
        if total_length % 2 == 0 {
            let a = Solution::find_smallest_in_sorted_arrays(
                &nums1,
                &nums2,
                (total_length / 2 + 1) as i32,
            );
            let b =
                Solution::find_smallest_in_sorted_arrays(&nums1, &nums2, (total_length / 2) as i32);
            return (a + b) as f64 / 2.0;
        } else {
            return Solution::find_smallest_in_sorted_arrays(
                &nums1,
                &nums2,
                (total_length / 2 + 1) as i32,
            ) as f64;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_median_sorted_arrays() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
