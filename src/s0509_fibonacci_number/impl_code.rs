pub struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        } else if n <= 2 {
            return 1;
        }
        let n = n as usize;
        let mut dp = vec![0; n];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..n {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n - 1]
    }

    pub fn fib_recur(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        Solution::fib_recur(n - 1) + Solution::fib_recur(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_fib() {
        assert_eq!(Solution::fib(2), 1);
        assert_eq!(Solution::fib(3), 2);
        assert_eq!(Solution::fib(4), 3);
    }

    #[test]
    fn test_fib_recur() {
        assert_eq!(Solution::fib_recur(2), 1);
        assert_eq!(Solution::fib_recur(3), 2);
        assert_eq!(Solution::fib_recur(4), 3);
    }
}
