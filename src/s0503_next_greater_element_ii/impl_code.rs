pub struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![-1; n];
        let mut stack: Vec<usize> = vec![];
        for i in 0..n * 2 - 1 {
            while !stack.is_empty() && nums[*stack.last().unwrap()] < nums[i % n] {
                ans[stack.pop().unwrap()] = nums[i % n];
            }
            stack.push(i % n);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_greater_elements() {
        assert_eq!(
            Solution::next_greater_elements(vec![1, 2, 1]),
            vec![2, -1, 2]
        );
    }
}
