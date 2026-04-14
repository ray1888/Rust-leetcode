pub struct Solution;

impl Solution {
    pub fn is_monotonic(a: Vec<i32>) -> bool {
        let mut inc = true;
        let mut dec = true;
        for i in 0..a.len().saturating_sub(1) {
            if a[i] > a[i + 1] {
                inc = false;
            }
            if a[i] < a[i + 1] {
                dec = false;
            }
        }
        inc || dec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_monotonic() {
        assert_eq!(Solution::is_monotonic(vec![1, 2, 3, 3]), true);
        assert_eq!(Solution::is_monotonic(vec![6, 5, 4, 4]), true);
    }
}
