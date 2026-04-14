pub struct Solution;

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let mut count = 0;
        let mut n = n;
        while n > 0 {
            n /= 5;
            count += n;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trailing_zeroes() {
        assert_eq!(Solution::trailing_zeroes(4), 0);
        assert_eq!(Solution::trailing_zeroes(5), 1);
    }
}
