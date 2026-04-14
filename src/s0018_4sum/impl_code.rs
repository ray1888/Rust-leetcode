pub struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return vec![];
        }
        let n = nums.len();
        let mut result: Vec<Vec<i32>> = Vec::new();
        nums.sort();
        let mut p = n - 1;
        while p >= 3 {
            if 4 * nums[p] < target {
                break;
            }
            let mut k = p - 1;
            while k >= 2 {
                if 3 * nums[k] + nums[p] < target {
                    break;
                }
                let new_target = target - nums[p] - nums[k];
                let mut i = 0usize;
                let mut j = k - 1;
                while i < j {
                    if nums[i] + nums[j] == new_target {
                        result.push(vec![nums[i], nums[j], nums[k], nums[p]]);
                        while i < j && nums[i + 1] == nums[i] {
                            i += 1;
                        }
                        while i < j && nums[j] == nums[j - 1] {
                            j -= 1;
                        }
                        i += 1;
                        j -= 1;
                    } else if nums[i] + nums[j] > new_target {
                        j -= 1;
                    } else {
                        i += 1;
                    }
                }
                while k >= 2 && nums[k - 1] == nums[k] {
                    k -= 1;
                }
                k -= 1;
            }
            while p >= 2 && nums[p - 1] == nums[p] {
                p -= 1;
            }
            p -= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_four_sum() {
        let cases = vec![
            (vec![-3, 0, 1, 2], 0, vec![vec![-3, 0, 1, 2]]),
            (
                vec![1, 0, -1, 0, -3, 2],
                0,
                vec![vec![-3, 0, 1, 2], vec![-1, 0, 0, 1]],
            ),
        ];

        for (nums, target, expected) in cases {
            let actual = Solution::four_sum(nums, target);
            assert_eq!(actual.len(), expected.len(), "length does not match");
            let mut counter = 0;
            for val in &actual {
                for exp in &expected {
                    let mut result = true;
                    for j in 0..4 {
                        if val[j] != exp[j] {
                            result = false;
                            break;
                        }
                    }
                    if result {
                        counter += 1;
                        break;
                    }
                }
            }
            assert_eq!(counter, actual.len(), "the match seq is not right");
        }
    }
}
