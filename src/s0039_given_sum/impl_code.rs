pub struct Solution;

impl Solution {
    fn combine_dfs(
        candidates: &Vec<i32>,
        start: usize,
        target: i32,
        tmp: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target < 0 {
            return;
        }
        if target == 0 {
            result.push(tmp.clone());
            return;
        }
        for i in start..candidates.len() {
            if candidates[i] > target {
                break;
            }
            tmp.push(candidates[i]);
            Self::combine_dfs(candidates, i, target - candidates[i], tmp, result);
            tmp.pop();
        }
    }

    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if candidates.is_empty() {
            return vec![];
        }
        candidates.sort();
        let mut result = vec![];
        let mut tmp = vec![];
        Self::combine_dfs(&candidates, 0, target, &mut tmp, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_combination_sum() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let expected: Vec<Vec<i32>> = vec![vec![2, 2, 3], vec![7]];
        assert_eq!(Solution::combination_sum(candidates, target), expected);
    }
}
