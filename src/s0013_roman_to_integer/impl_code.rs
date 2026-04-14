pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        use std::collections::HashMap;

        let mut basic = HashMap::new();
        basic.insert('I', 1);
        basic.insert('V', 5);
        basic.insert('X', 10);
        basic.insert('L', 50);
        basic.insert('C', 100);
        basic.insert('D', 500);
        basic.insert('M', 1000);

        let bytes: Vec<u8> = s.bytes().collect();
        let mut result = *basic.get(&(bytes[bytes.len() - 1] as char)).unwrap();

        for i in (0..bytes.len() - 1).rev() {
            let curr = *basic.get(&(bytes[i] as char)).unwrap();
            let next = *basic.get(&(bytes[i + 1] as char)).unwrap();
            if curr < next {
                result -= curr;
            } else {
                result += curr;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        let tests = vec![
            ("IV", 4),
            ("VI", 6),
            ("XL", 40),
            ("LX", 60),
            ("XC", 90),
            ("CX", 110),
            ("CMDXL", 1440),
        ];

        for (input, expected) in tests {
            assert_eq!(Solution::roman_to_int(input.to_string()), expected);
        }
    }
}
