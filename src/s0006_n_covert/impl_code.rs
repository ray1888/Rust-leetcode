pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows == 1 {
            return s;
        }
        let mut rows = vec![String::new(); num_rows];
        let period = num_rows * 2 - 2;
        for (i, c) in s.chars().enumerate() {
            let mod_val = i % period;
            if mod_val < num_rows {
                rows[mod_val].push(c);
            } else {
                rows[period - mod_val].push(c);
            }
        }
        rows.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );
    }
}
