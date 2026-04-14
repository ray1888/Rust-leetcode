use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut n = n;
        let mut seen = HashSet::new();
        while n != 1 && !seen.contains(&n) {
            seen.insert(n);
            n = Self::step(n);
        }
        n == 1
    }

    fn step(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            let digit = n % 10;
            sum += digit * digit;
            n /= 10;
        }
        sum
    }

    pub fn is_happy_two_pointer(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        let mut fast = n;
        let mut slow = n;
        loop {
            fast = Self::transform(Self::transform(fast));
            slow = Self::transform(slow);
            if fast == 1 {
                return true;
            }
            if fast == slow {
                return false;
            }
        }
    }

    fn transform(mut input: i32) -> i32 {
        let mut sum = 0;
        while input != 0 {
            let a = input % 10;
            sum += a * a;
            input /= 10;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_happy() {
        assert_eq!(Solution::is_happy(19), true);
        assert_eq!(Solution::is_happy(2), false);
    }

    #[test]
    fn test_is_happy_two_pointer() {
        assert_eq!(Solution::is_happy_two_pointer(28), true);
        assert_eq!(Solution::is_happy_two_pointer(2), false);
    }
}
