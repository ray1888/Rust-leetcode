pub struct Solution;

impl Solution {
    fn permuteRec(arr: &mut Vec<i32>, index: usize, result: &mut Vec<Vec<i32>>) {
        let length = arr.len();
        if index == length {
            result.push(arr.to_vec());
            return;
        }

        for i in index..length {
            arr.swap(i, index);
            Self::permuteRec(arr, index + 1, result);
            arr.swap(i, index);
        }
    }

    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 0 {
            return vec![vec![]];
        }
        let mut result = vec![];
        Solution::permuteRec(&mut nums, 0, &mut result);
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute() {
        let mut result = Solution::permute(vec![1, 2, 3]);
        result.sort();
        assert_eq!(
            result,
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1],
            ]
        );
    }
}
