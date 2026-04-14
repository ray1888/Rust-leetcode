pub struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        if s.is_empty() {
            return vec![];
        }
        let mut result = vec![];
        let mut segment = vec![0; 4];
        let s = s.into_bytes();
        Self::dfs_ip(&s, 0, 0, &mut segment, &mut result);
        result
    }

    fn dfs_ip(
        s: &Vec<u8>,
        segment_id: usize,
        seg_start_index: usize,
        segment: &mut Vec<i32>,
        result: &mut Vec<String>,
    ) {
        if segment_id == 4 {
            if seg_start_index == s.len() {
                let ipaddr = format!(
                    "{}.{}.{}.{}",
                    segment[0], segment[1], segment[2], segment[3]
                );
                result.push(ipaddr);
            }
            return;
        }
        if seg_start_index == s.len() {
            return;
        }

        if s[seg_start_index] == b'0' {
            segment[segment_id] = 0;
            Self::dfs_ip(s, segment_id + 1, seg_start_index + 1, segment, result);
            return;
        }

        let mut addr = 0;
        for seg_end in seg_start_index..s.len() {
            addr = addr * 10 + (s[seg_end] - b'0') as i32;
            if addr > 0 && addr <= 255 {
                segment[segment_id] = addr;
                Self::dfs_ip(s, segment_id + 1, seg_end + 1, segment, result);
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_restore_ip_addresses() {
        let s = "010010".to_string();
        let mut result = Solution::restore_ip_addresses(s);
        result.sort();
        let mut expected = vec!["0.10.0.10".to_string(), "0.100.1.0".to_string()];
        expected.sort();
        assert_eq!(result, expected);
    }
}
