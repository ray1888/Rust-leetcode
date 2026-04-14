pub struct Solution;

impl Solution {
    pub fn split_brutal_force(arr: &[i32], k: i32) -> i32 {
        if arr.is_empty() {
            return 0;
        }
        let mut max_value = 0;
        for &item in arr {
            max_value = max_value.max(item);
        }
        let mut m = 1;
        let mut max_count = 0;
        while m < max_value {
            let mut count = 0;
            for &item in arr {
                count += item / m;
            }
            if count >= k {
                max_count = max_count.max(m);
            }
            m += 1;
        }
        max_count
    }

    pub fn split_half_cut(arr: &[i32], k: i32) -> i32 {
        if arr.is_empty() {
            return 0;
        }
        let mut r = -1;
        let mut l = 1;
        for &item in arr {
            r = r.max(item);
        }

        while l < r {
            let mid = l + (r - l) / 2;
            let res: i32 = arr.iter().map(|&x| x / mid).sum();
            if res >= k {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_brutal_force() {
        assert_eq!(Solution::split_brutal_force(&[4, 7, 2, 10, 5], 5), 4);
    }

    #[test]
    fn test_split_half_cut() {
        assert_eq!(Solution::split_half_cut(&[4, 7, 2, 10, 5], 5), 4);
    }
}
