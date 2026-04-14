pub struct Solution;

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        nums.sort();
        let length = nums.len();
        let mut count = 0;
        for i in 0..length - 2 {
            let mut k = i + 2;
            for j in i + 1..length - 1 {
                if nums[i] == 0 {
                    continue;
                }
                while k < length && nums[i] + nums[j] > nums[k] {
                    k += 1;
                }
                count += (k - j - 1) as i32;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_number() {
        assert_eq!(Solution::triangle_number(vec![2, 2, 3, 4]), 3);
        assert_eq!(Solution::triangle_number(vec![0, 0, 0]), 0);
        assert_eq!(Solution::triangle_number(vec![0, 1, 0, 1]), 0);
    }
}
