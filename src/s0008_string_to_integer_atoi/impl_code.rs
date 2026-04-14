pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut state = State::Start;
        let mut negative = false;
        let mut result = 0i64;

        for b in s.bytes() {
            match state {
                State::Start => {
                    if b == b' ' {
                        continue;
                    } else if b >= b'0' && b <= b'9' {
                        result = result * 10 + (b - b'0') as i64;
                        state = State::FoundNumber;
                    } else if b == b'-' || b == b'+' {
                        negative = b == b'-';
                        state = State::FoundSymbol;
                    } else {
                        break;
                    }
                }
                State::FoundSymbol | State::FoundNumber => {
                    if b >= b'0' && b <= b'9' {
                        result = result * 10 + (b - b'0') as i64;
                        state = State::FoundNumber;
                        if result >= (i32::MAX as i64) + 1 {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        if result >= (i32::MAX as i64) + 1 {
            if negative {
                i32::MIN
            } else {
                i32::MAX
            }
        } else {
            if negative {
                -(result as i32)
            } else {
                result as i32
            }
        }
    }
}

enum State {
    Start,
    FoundSymbol,
    FoundNumber,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_atoi() {
        let cases = vec![
            ("42", 42),
            ("-92", -92),
            ("+-12", 0),
            ("-+12", 0),
            ("-91283472332", i32::MIN),
            ("words and 987", 0),
        ];

        for (input, expected) in cases {
            assert_eq!(
                Solution::my_atoi(input.to_string()),
                expected,
                "failed on input: {}",
                input
            );
        }
    }
}
