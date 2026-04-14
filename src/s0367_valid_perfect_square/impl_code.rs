pub struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut low = 1;
        let mut high = num;
        while low <= high {
            let mid = low + (high - low) / 2;
            let square = mid as i64 * mid as i64;
            if square == num as i64 {
                return true;
            } else if square > num as i64 {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_perfect_square() {
        assert_eq!(Solution::is_perfect_square(9), true);
        assert_eq!(Solution::is_perfect_square(8), false);
    }
}
