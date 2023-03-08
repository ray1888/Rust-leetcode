pub struct Solution;


impl Solution {
    fn permuteRec(arr: &mut Vec<i32>, index: usize,result: &mut Vec<Vec<i32>>){
        let length = arr.len();
        if index == length {
            result.push(arr.to_vec());
            return;
        }

        for i in index..length{
            arr.swap(i, index);
            Self::permuteRec(arr, index+1, result);
            arr.swap(i, index);
        }
    }

    pub fn permute(mut nums:  Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 0 {
            return vec![vec![]];
        }
        let mut result = vec![];
        Solution::permuteRec(&mut nums, 0, &mut result);
        return result;
    }
}