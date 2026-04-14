pub struct Solution;

impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        if arr.is_empty() {
            return 0;
        }
        let mut max_length = 1;
        let mut left = 0usize;
        let mut right = 0usize;

        while right < arr.len() - 1 {
            if left == right {
                if arr[left] == arr[left + 1] {
                    left += 1;
                }
                right += 1;
            } else {
                if arr[right - 1] < arr[right] && arr[right] > arr[right + 1] {
                    right += 1;
                } else if arr[right - 1] > arr[right] && arr[right] < arr[right + 1] {
                    right += 1;
                } else {
                    left = right;
                }
            }
            max_length = max_length.max(right - left + 1);
        }
        max_length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_turbulence_size() {
        assert_eq!(
            Solution::max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9]),
            5
        );
    }
}
