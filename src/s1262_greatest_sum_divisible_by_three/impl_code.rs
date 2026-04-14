pub struct Solution;

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut state = vec![0, i32::MIN, i32::MIN];
        for item in nums {
            let rem = item % 3;
            if rem == 0 {
                state = vec![state[0] + item, state[1] + item, state[2] + item];
            } else if rem == 1 {
                let a = state[2] + item;
                let b = state[0];
                let c = state[0] + item;
                let d = state[1];
                let e = state[1] + item;
                let f = state[2];
                state = vec![a.max(b), c.max(d), e.max(f)];
            } else {
                let a = state[1] + item;
                let b = state[0];
                let c = state[2] + item;
                let d = state[1];
                let e = state[0] + item;
                let f = state[2];
                state = vec![a.max(b), c.max(d), e.max(f)];
            }
        }
        state[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum_div_three() {
        assert_eq!(Solution::max_sum_div_three(vec![3, 6, 5, 1, 8]), 18);
    }
}
