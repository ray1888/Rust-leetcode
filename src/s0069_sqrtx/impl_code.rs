pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut start = 1i64;
        let mut end = x as i64;
        while start <= end {
            let mid = start + (end - start) / 2;
            let square_mid = mid * mid;
            if square_mid > x as i64 {
                end = mid - 1;
            } else if square_mid < x as i64 {
                start = mid + 1;
            } else {
                return mid as i32;
            }
        }
        end as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_sqrt() {
        let cases = vec![(8, 2), (4, 2)];
        for (x, expected) in cases {
            assert_eq!(Solution::my_sqrt(x), expected);
        }
    }
}
