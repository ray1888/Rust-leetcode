pub struct Solution;

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        let mut i = 0;
        let length = arr.len();
        while i + 1 < length && arr[i] < arr[i + 1] {
            i += 1;
        }
        if i == 0 || i == length - 1 {
            return false;
        }
        while i + 1 < length && arr[i] > arr[i + 1] {
            i += 1;
        }
        i == length - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_mountain_array() {
        assert_eq!(Solution::valid_mountain_array(vec![0, 3, 2, 1]), true);
        assert_eq!(Solution::valid_mountain_array(vec![3, 5, 5]), false);
        assert_eq!(Solution::valid_mountain_array(vec![2, 1]), false);
    }
}
