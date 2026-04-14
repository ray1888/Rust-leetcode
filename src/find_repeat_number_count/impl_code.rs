pub struct Solution;

impl Solution {
    fn b_search(arr: &[i32], target: i32, is_left: bool) -> usize {
        let mut pos = 0;
        let mut i = 0;
        let mut j = arr.len() as i32 - 1;
        while i <= j {
            let mid = i + (j - i) / 2;
            if arr[mid as usize] > target {
                j = mid - 1;
            } else if arr[mid as usize] < target {
                i = mid + 1;
            } else {
                pos = mid as usize;
                if is_left {
                    j = mid - 1;
                } else {
                    i = mid + 1;
                }
            }
        }
        pos
    }

    pub fn find_repeat_count_o_log_n(arr: Vec<i32>, repeat_num: i32) -> i32 {
        let left_index = Solution::b_search(&arr, repeat_num, true);
        let right_index = Solution::b_search(&arr, repeat_num, false);
        (right_index - left_index + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_repeat_count_o_log_n() {
        assert_eq!(
            Solution::find_repeat_count_o_log_n(vec![1, 2, 2, 2, 5, 6], 2),
            3
        );
        assert_eq!(
            Solution::find_repeat_count_o_log_n(vec![1, 2, 3, 5, 5], 5),
            2
        );
    }
}
