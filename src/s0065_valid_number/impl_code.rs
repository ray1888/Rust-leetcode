pub struct Solution;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum State {
    Initial,
    IntSign,
    Integer,
    Point,
    PointWithoutInt,
    Fraction,
    Exp,
    ExpSign,
    ExpNumber,
    End,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum CharType {
    Number,
    Exp,
    Point,
    Sign,
    Space,
    Illegal,
}

fn to_char_type(ch: u8) -> CharType {
    match ch {
        b'0'..=b'9' => CharType::Number,
        b'e' | b'E' => CharType::Exp,
        b'.' => CharType::Point,
        b'+' | b'-' => CharType::Sign,
        b' ' => CharType::Space,
        _ => CharType::Illegal,
    }
}

impl Solution {
    pub fn is_number(s: String) -> bool {
        use std::collections::HashMap;

        let transfer: HashMap<State, HashMap<CharType, State>> = [
            (
                State::Initial,
                [
                    (CharType::Space, State::Initial),
                    (CharType::Number, State::Integer),
                    (CharType::Point, State::PointWithoutInt),
                    (CharType::Sign, State::IntSign),
                ]
                .iter()
                .cloned()
                .collect(),
            ),
            (
                State::IntSign,
                [
                    (CharType::Number, State::Integer),
                    (CharType::Point, State::PointWithoutInt),
                ]
                .iter()
                .cloned()
                .collect(),
            ),
            (
                State::Integer,
                [
                    (CharType::Number, State::Integer),
                    (CharType::Exp, State::Exp),
                    (CharType::Point, State::Point),
                    (CharType::Space, State::End),
                ]
                .iter()
                .cloned()
                .collect(),
            ),
            (
                State::Point,
                [
                    (CharType::Number, State::Fraction),
                    (CharType::Exp, State::Exp),
                    (CharType::Space, State::End),
                ]
                .iter()
                .cloned()
                .collect(),
            ),
            (
                State::PointWithoutInt,
                [(CharType::Number, State::Fraction)]
                    .iter()
                    .cloned()
                    .collect(),
            ),
            (
                State::Fraction,
                [
                    (CharType::Number, State::Fraction),
                    (CharType::Exp, State::Exp),
                    (CharType::Space, State::End),
                ]
                .iter()
                .cloned()
                .collect(),
            ),
            (
                State::Exp,
                [
                    (CharType::Number, State::ExpNumber),
                    (CharType::Sign, State::ExpSign),
                ]
                .iter()
                .cloned()
                .collect(),
            ),
            (
                State::ExpSign,
                [(CharType::Number, State::ExpNumber)]
                    .iter()
                    .cloned()
                    .collect(),
            ),
            (
                State::ExpNumber,
                [
                    (CharType::Number, State::ExpNumber),
                    (CharType::Space, State::End),
                ]
                .iter()
                .cloned()
                .collect(),
            ),
            (
                State::End,
                [(CharType::Space, State::End)].iter().cloned().collect(),
            ),
        ]
        .iter()
        .cloned()
        .collect();

        let mut state = State::Initial;
        for b in s.bytes() {
            let typ = to_char_type(b);
            match transfer.get(&state).and_then(|m| m.get(&typ)) {
                Some(&next_state) => state = next_state,
                None => return false,
            }
        }

        state == State::Integer
            || state == State::Point
            || state == State::Fraction
            || state == State::ExpNumber
            || state == State::End
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_number() {
        let cases = vec![
            ("+100", true),
            ("5e2", true),
            ("3.14", true),
            ("1a3.14", false),
            ("+-5", false),
            ("12e+5.4", false),
        ];

        for (input, expected) in cases {
            assert_eq!(
                Solution::is_number(input.to_string()),
                expected,
                "failed on input: {}",
                input
            );
        }
    }
}
