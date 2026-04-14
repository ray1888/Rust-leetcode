pub struct Solution;

impl Solution {
    pub fn min_cost_ii(costs: Vec<Vec<i32>>) -> i32 {
        if costs.is_empty() {
            return 0;
        }
        let n = costs.len();
        let k = costs[0].len();
        let mut d = vec![vec![0; k]; n + 1];
        for i in 1..=n {
            for j in 0..k {
                let mut min_v = i32::MAX;
                for c in 0..k {
                    if c != j {
                        min_v = min_v.min(d[i - 1][c]);
                    }
                }
                d[i][j] = min_v + costs[i - 1][j];
            }
        }
        let mut min_value = i32::MAX;
        for i in 0..k {
            min_value = min_value.min(d[n][i]);
        }
        min_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost_ii() {
        let costs = vec![vec![8, 2, 4], vec![5, 7, 3], vec![9, 1, 6], vec![4, 1, 9]];
        assert_eq!(Solution::min_cost_ii(costs), 10);
    }
}
