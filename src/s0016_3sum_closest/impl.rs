pub struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
         if nums.len() == 0 {
            return 0;
        }
        let length = nums.len();
        let mut result :i32 = 0;
        let mut k = length-1;
        let mut min = std::i32::MAX;
        nums.sort();
        while  k >=2 {
            let mut i = 0;
            let mut j = k-1;
            while i<j {
                let sum = nums[i] + nums[j] + nums[k];
                if target == sum {
                    return sum
                } else if target < sum{
                    j -= 1
                } else {
                    i += 1
                }
                let mut diff = target - sum;
                if diff < 0 {
                    diff = -1 * diff 
                }
                if diff < min {
                    result = sum;
                    min = diff;
                }
            }
            k-=1;
        }
        return result;
    }
}