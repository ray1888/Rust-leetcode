pub struct Solution;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![vec![]];
        }
        nums.sort();
        let mut result: Vec<Vec<i32>> = vec![];
        let mut tmp = vec![];
        Self::dfs(&nums, &mut tmp, 0, &mut result);
        result
    }

    fn dfs(nums: &[i32], tmp: &mut Vec<i32>, start: usize, result: &mut Vec<Vec<i32>>) {
        result.push(tmp.clone());
        for i in start..nums.len() {
            if i > start && nums[i] == nums[i - 1] {
                continue;
            }
            tmp.push(nums[i]);
            Self::dfs(nums, tmp, i + 1, result);
            tmp.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets_with_dup() {
        let mut result = Solution::subsets_with_dup(vec![1, 2, 2]);
        result.sort();
        assert_eq!(
            result,
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
                vec![2],
                vec![2, 2],
            ]
        );
    }
}
