pub struct Solution;

impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut res: Vec<i32> = Vec::new();
        let start = if k > nums2.len() { k - nums2.len() } else { 0 };
        let end = k.min(nums1.len());
        for i in start..=end {
            let s1 = Self::max_subsequence(&nums1, i);
            let s2 = Self::max_subsequence(&nums2, k - i);
            let merged = Self::merge(&s1, &s2);
            if Self::lexicographical_less(&res, &merged) {
                res = merged;
            }
        }
        res
    }

    fn max_subsequence(a: &[i32], k: usize) -> Vec<i32> {
        let mut s: Vec<i32> = Vec::new();
        let n = a.len();
        for (i, &v) in a.iter().enumerate() {
            while !s.is_empty() && s.len() + n - 1 - i >= k && v > *s.last().unwrap() {
                s.pop();
            }
            if s.len() < k {
                s.push(v);
            }
        }
        s
    }

    fn lexicographical_less(a: &[i32], b: &[i32]) -> bool {
        let len = a.len().min(b.len());
        for i in 0..len {
            if a[i] != b[i] {
                return a[i] < b[i];
            }
        }
        a.len() < b.len()
    }

    fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
        let mut merged = Vec::with_capacity(a.len() + b.len());
        let mut a = a.to_vec();
        let mut b = b.to_vec();
        for _ in 0..(a.len() + b.len()) {
            if Self::lexicographical_less(&a, &b) {
                merged.push(b.remove(0));
            } else {
                merged.push(a.remove(0));
            }
        }
        merged
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_number() {
        assert_eq!(
            Solution::max_number(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5),
            vec![9, 8, 6, 5, 3]
        );
        assert_eq!(
            Solution::max_number(vec![6, 7], vec![6, 0, 4], 5),
            vec![6, 7, 6, 0, 4]
        );
    }
}
