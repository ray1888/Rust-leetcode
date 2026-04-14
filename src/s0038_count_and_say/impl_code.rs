pub struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n < 1 {
            return "".to_string();
        }
        let mut s = "1".to_string();
        for _ in 1..n {
            let mut count = 1;
            let mut sb = String::new();
            let bytes = s.as_bytes();
            for p in 0..bytes.len() {
                if p + 1 < bytes.len() && bytes[p] == bytes[p + 1] {
                    count += 1;
                } else {
                    sb.push_str(&count.to_string());
                    sb.push(bytes[p] as char);
                    count = 1;
                }
            }
            s = sb;
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_and_say() {
        assert_eq!(Solution::count_and_say(1), "1");
        assert_eq!(Solution::count_and_say(2), "11");
        assert_eq!(Solution::count_and_say(3), "21");
        assert_eq!(Solution::count_and_say(4), "1211");
        assert_eq!(Solution::count_and_say(5), "111221");
    }
}
