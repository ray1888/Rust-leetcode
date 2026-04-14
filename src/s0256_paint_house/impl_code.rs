pub struct Solution;

use std::cmp::min;

impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        if costs.is_empty() {
            return 0;
        }
        let n = costs.len();
        let mut d = vec![vec![0; 3]; n];
        d[0][0] = costs[0][0];
        d[0][1] = costs[0][1];
        d[0][2] = costs[0][2];
        for i in 1..n {
            d[i][0] = min(d[i - 1][1], d[i - 1][2]) + costs[i][0];
            d[i][1] = min(d[i - 1][0], d[i - 1][2]) + costs[i][1];
            d[i][2] = min(d[i - 1][0], d[i - 1][1]) + costs[i][2];
        }
        min(min(d[n - 1][0], d[n - 1][1]), d[n - 1][2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost() {
        let costs = vec![vec![8, 2, 4], vec![5, 7, 3], vec![9, 1, 6], vec![4, 1, 9]];
        assert_eq!(Solution::min_cost(costs), 10);
    }
}
