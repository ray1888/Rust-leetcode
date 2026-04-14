pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut pa = m - 1;
        let mut pb = n - 1;
        let mut tail = m + n - 1;
        while pa >= 0 || pb >= 0 {
            let cur = if pa == -1 {
                let v = nums2[pb as usize];
                pb -= 1;
                v
            } else if pb == -1 {
                let v = nums1[pa as usize];
                pa -= 1;
                v
            } else if nums1[pa as usize] > nums2[pb as usize] {
                let v = nums1[pa as usize];
                pa -= 1;
                v
            } else {
                let v = nums2[pb as usize];
                pb -= 1;
                v
            };
            nums1[tail as usize] = cur;
            tail -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }
}
