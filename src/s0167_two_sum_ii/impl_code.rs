pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0usize;
        let mut j = numbers.len().saturating_sub(1);
        while i < j {
            let sum = numbers[i] + numbers[j];
            if sum > target {
                j -= 1;
            } else if sum < target {
                i += 1;
            } else {
                return vec![i as i32 + 1, j as i32 + 1];
            }
        }
        vec![-1, -1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_ii() {
        let cases = vec![
            (vec![1, 3, 5, 8], 2, vec![-1, -1]),
            (vec![1, 3, 5, 6], 4, vec![1, 2]),
            (vec![1, 3, 5, 6], 5, vec![-1, -1]),
            (vec![1, 3, 5, 8], 9, vec![1, 4]),
        ];
        for (numbers, target, expected) in cases {
            assert_eq!(Solution::two_sum(numbers, target), expected);
        }
    }
}
