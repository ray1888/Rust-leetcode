pub struct Solution;

impl Solution {
    fn combine_sub(n: i32, k: i32, start: i32, elem: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if k == 0 {
            result.push(elem.clone());
            return;
        }
        for i in start..=n - k + 1 {
            elem.push(i);
            Self::combine_sub(n, k - 1, i + 1, elem, result);
            elem.pop();
        }
    }

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        if k > n {
            return vec![];
        }
        let mut result = vec![];
        let mut elem = vec![];
        Self::combine_sub(n, k, 1, &mut elem, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_combine() {
        let expected: Vec<Vec<i32>> = vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
        ];
        assert_eq!(Solution::combine(4, 2), expected);
    }
}
