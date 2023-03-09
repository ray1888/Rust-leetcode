pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 0 {
            return vec![];
        }
        let length = nums.len();
        let mut result :Vec<Vec<i32>> = Vec::new();
        let mut k = length-1;
        nums.sort();
        while  k >=2 {
            if nums[k] < 0 {
                break;
            }
            let target = -nums[k];
            let mut i = 0;
            let mut j = k-1;
            while i < j {
                if nums[i] + nums[j]  == target{
                    let tmpResult:Vec<i32> = vec![nums[i], nums[j], nums[k]];
                    result.push(tmpResult);
                    while i<j && nums[i] == nums[i+1]{
                        i+=1;
                    }
                    while i<j && nums[j] == nums[j-1]{
                        j-=1;
                    }
                    i+=1;
                    j-=1;
                }  else if nums[i] + nums[j] < target {
                    i+=1;
                } else {
                    j-=1;
                }
            }
            while k > 2 && nums[k] == nums[k-1]{
                k-=1;
            }
            k-=1;
        }
        return result;
    }
}