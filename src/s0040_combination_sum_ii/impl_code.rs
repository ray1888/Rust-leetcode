pub struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if candidates.is_empty() {
            return vec![];
        }
        candidates.sort();
        let mut result = vec![];
        let mut tmp = vec![];
        let mut used = vec![false; candidates.len()];
        Self::combine_dfs(&candidates, 0, target, &mut tmp, &mut used, &mut result);
        result
    }

    fn combine_dfs(
        candidates: &Vec<i32>,
        start: usize,
        target: i32,
        tmp: &mut Vec<i32>,
        used: &mut Vec<bool>,
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
            if i > 0 && candidates[i] == candidates[i - 1] && !used[i - 1] {
                continue;
            }
            tmp.push(candidates[i]);
            used[i] = true;
            Self::combine_dfs(candidates, i + 1, target - candidates[i], tmp, used, result);
            used[i] = false;
            tmp.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_combination_sum2() {
        let candidates = vec![2, 5, 2, 1, 2];
        let target = 5;
        let mut result = Solution::combination_sum2(candidates, target);
        result.sort();
        let expected = vec![vec![1, 2, 2], vec![5]];
        assert_eq!(result, expected);
    }
}
