pub struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut l = 1i32;
        let mut r = (n - 1) as i32;
        let mut ans = -1;
        while l <= r {
            let mid = (l + r) >> 1;
            let mut cnt = 0;
            for i in 0..n {
                if nums[i] <= mid {
                    cnt += 1;
                }
            }
            if cnt <= mid {
                l = mid + 1;
            } else {
                r = mid - 1;
                ans = mid;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicate() {
        let cases = vec![(vec![1, 3, 4, 2, 2], 2)];
        for (nums, expected) in cases {
            assert_eq!(Solution::find_duplicate(nums), expected);
        }
    }
}
