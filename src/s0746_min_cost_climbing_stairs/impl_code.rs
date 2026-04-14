pub struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        if cost.is_empty() {
            return 0;
        }
        let mut first = cost[0];
        let mut second = cost[1];
        for i in 2..cost.len() {
            let cur = first.min(second) + cost[i];
            first = second;
            second = cur;
        }
        first.min(second)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost_climbing_stairs() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
