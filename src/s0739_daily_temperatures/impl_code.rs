pub struct Solution;

impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        if t.is_empty() {
            return t;
        }
        let n = t.len();
        let mut stack: Vec<usize> = Vec::with_capacity(n);
        let mut result = vec![0; n];
        for i in 0..n {
            while !stack.is_empty() && t[i] > t[*stack.last().unwrap()] {
                let idx = stack.pop().unwrap();
                result[idx] = (i - idx) as i32;
            }
            stack.push(i);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daily_temperatures() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }
}
