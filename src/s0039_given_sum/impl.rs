pub struct Solution;

impl Solution {
    fn combination(candidates:&Vec<i32>, start: usize, target:i32, result: &mut Vec<Vec<i32>>, elem:&mut Vec<i32>) {
        if target<0 {
            return
        } else if target == 0 {
            result.push(elem.to_vec());
            return 
        } else {
            for i in start..candidates.len(){
                if candidates[i] > target{
                    break
                }
                elem.push(candidates[i]);
                Self::combination(candidates,i,target-candidates[i],result, elem);
                elem.pop();
            }
        }
    }

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if candidates.len() ==0 && target != 0 {
            return vec![];
        } else if candidates.len() != 0 && target == 0{
            return vec![];
        }
        let mut result = vec![];
        let mut elem:Vec<i32> = vec![];
        Self::combination(&candidates, 0, target, &mut result,&mut elem);
        result
    }
}