pub struct Solution;

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut digits: Vec<u8> = num.to_string().into_bytes();
        let mut indexes = [0; 10];
        for (i, &b) in digits.iter().enumerate() {
            indexes[(b - b'0') as usize] = i;
        }
        for (i, &b) in digits.iter().enumerate() {
            let current = (b - b'0') as usize;
            for d in (current + 1..=9).rev() {
                if indexes[d] > i {
                    digits.swap(i, indexes[d]);
                    return String::from_utf8(digits).unwrap().parse().unwrap();
                }
            }
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_swap() {
        assert_eq!(Solution::maximum_swap(2736), 7236);
    }
}
