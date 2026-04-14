pub struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n <= 2 {
            return 0;
        }
        let n = n as usize;
        let mut primes = vec![true; n];
        primes[0] = false;
        primes[1] = false;
        let sqrt_n = (n as f64).sqrt() as usize;
        for i in 2..=sqrt_n {
            if primes[i] {
                let mut j = i * i;
                while j < n {
                    primes[j] = false;
                    j += i;
                }
            }
        }
        primes.iter().filter(|&&v| v).count() as i32
    }

    pub fn count_primes_fast(n: i32) -> i32 {
        if n <= 2 {
            return 0;
        }
        let n = n as usize;
        let mut composites = vec![false; n];
        let mut count = 1;
        let sqrt_n = (n as f64).sqrt() as usize;
        let mut i = 3;
        while i < n {
            if !composites[i] {
                count += 1;
                if i <= sqrt_n {
                    let mut j = i * i;
                    while j < n {
                        composites[j] = true;
                        j += 2 * i;
                    }
                }
            }
            i += 2;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_primes() {
        assert_eq!(Solution::count_primes(8), 4);
    }

    #[test]
    fn test_count_primes_fast() {
        assert_eq!(Solution::count_primes_fast(8), 4);
    }
}
