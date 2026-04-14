pub struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut i = 0usize;
        let mut j = 0usize;
        let v1 = version1.as_bytes();
        let v2 = version2.as_bytes();
        while i < v1.len() || j < v2.len() {
            let mut num1 = 0i32;
            let mut num2 = 0i32;
            while i < v1.len() && v1[i] != b'.' {
                num1 = num1 * 10 + (v1[i] - b'0') as i32;
                i += 1;
            }
            while j < v2.len() && v2[j] != b'.' {
                num2 = num2 * 10 + (v2[j] - b'0') as i32;
                j += 1;
            }
            if num1 > num2 {
                return 1;
            } else if num1 < num2 {
                return -1;
            }
            i += 1;
            j += 1;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_version() {
        assert_eq!(
            Solution::compare_version("1.01".to_string(), "1.001".to_string()),
            0
        );
        assert_eq!(
            Solution::compare_version("1.01".to_string(), "1.002".to_string()),
            -1
        );
        assert_eq!(
            Solution::compare_version("1.02".to_string(), "1.001".to_string()),
            1
        );
    }
}
