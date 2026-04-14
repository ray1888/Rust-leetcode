pub struct Solution;

use std::cmp::min;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return "".to_string();
        }
        let mut min_length = strs[0].len();
        let work_length = strs.len();

        for i in 1..strs.len() {
            min_length = min(min_length, strs[i].len());
        }

        let mut common_index = 0;
        for i in 0..min_length {
            let elem = strs[0].chars().nth(i).unwrap();
            let mut break_flag = false;
            for j in 1..work_length {
                if strs[j].chars().nth(i).unwrap() != elem {
                    break_flag = true;
                    break;
                }
            }
            if break_flag {
                break;
            }
            common_index += 1;
        }
        if common_index > 0 {
            return strs[0][..common_index].to_string();
        }
        return "".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "car".to_string(),
                "care".to_string(),
                "cat".to_string()
            ]),
            "ca"
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "car".to_string(),
                "bus".to_string(),
                "boot".to_string()
            ]),
            ""
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "car".to_string(),
                "car".to_string(),
                "car".to_string()
            ]),
            "car"
        );
    }
}
