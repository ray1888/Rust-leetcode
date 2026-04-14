pub struct Solution;

impl Solution {
    fn is_valid_two_digit(ss: &Vec<char>, i: usize) -> bool {
        (ss[i - 2] == '1' && (ss[i - 1] >= '0' && ss[i - 1] <= '9'))
            || (ss[i - 2] == '2' && ('0' <= ss[i - 1] && ss[i - 1] <= '6'))
    }

    pub fn num_decodings(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let ss: Vec<char> = s.chars().collect();
        let n = s.len() + 1;
        let mut d: Vec<i32> = vec![0; n];
        d[0] = 1;
        if ss[0] != '0' {
            d[1] = 1;
        }
        for i in 2..n {
            if ss[i - 1] != '0' {
                d[i] += d[i - 1];
            }
            if Solution::is_valid_two_digit(&ss, i) {
                d[i] += d[i - 2];
            }
        }
        d[s.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_num_decodings() {
        assert_eq!(Solution::num_decodings("12".to_string()), 2);
        assert_eq!(Solution::num_decodings("124".to_string()), 3);
    }
}
