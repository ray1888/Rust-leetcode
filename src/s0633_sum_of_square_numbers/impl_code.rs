pub struct Solution;

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        Self::sum_of_square_number_two_pointer(c)
    }

    fn is_square(input: i32) -> bool {
        let x = (input as f64).sqrt() as i32;
        x * x == input
    }

    pub fn sum_of_square_number_by_diff(input: i32) -> bool {
        if input < 0 {
            return false;
        }
        let sqrt_c = (input as f64).sqrt() as i32;
        for i in 0..=sqrt_c {
            if Self::is_square(input - i * i) {
                return true;
            }
        }
        false
    }

    pub fn sum_of_square_number_by_map(input: i32) -> bool {
        if input < 0 {
            return false;
        }
        let sqrt_c = (input as f64).sqrt() as i32;
        use std::collections::HashSet;
        let mut square_set = HashSet::new();
        for i in 0..=sqrt_c {
            let square = i * i;
            square_set.insert(square);
            if square_set.contains(&(input - square)) {
                return true;
            }
        }
        false
    }

    pub fn sum_of_square_number_two_pointer(input: i32) -> bool {
        if input < 0 {
            return false;
        }
        let mut i = 0i64;
        let mut j = (input as f64).sqrt() as i64;
        let target = input as i64;
        while i <= j {
            let sum_square = i * i + j * j;
            if sum_square == target {
                return true;
            } else if sum_square < target {
                i += 1;
            } else {
                j -= 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_square_number_by_diff() {
        assert_eq!(Solution::sum_of_square_number_by_diff(8), true);
        assert_eq!(Solution::sum_of_square_number_by_diff(7), false);
    }

    #[test]
    fn test_sum_of_square_number_by_map() {
        assert_eq!(Solution::sum_of_square_number_by_map(8), true);
        assert_eq!(Solution::sum_of_square_number_by_map(7), false);
    }

    #[test]
    fn test_sum_of_square_number_two_pointer() {
        assert_eq!(Solution::sum_of_square_number_two_pointer(8), true);
        assert_eq!(Solution::sum_of_square_number_two_pointer(7), false);
    }
}
