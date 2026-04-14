pub struct Solution;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n = n as usize;
        let k = k as usize;
        if n == 0 {
            return String::new();
        }

        let mut factorial = vec![1; n + 1];
        for i in 1..=n {
            factorial[i] = factorial[i - 1] * i;
        }

        let mut nums: Vec<i32> = (1..=n as i32).collect();
        let mut result = String::new();
        let mut k = k - 1;

        for i in 0..n {
            let fact = factorial[n - 1 - i];
            let index = k / fact;
            k = k % fact;
            result.push_str(&nums[index].to_string());
            nums.remove(index);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_get_permutation() {
        assert_eq!(Solution::get_permutation(3, 3), "213".to_string());
    }
}
