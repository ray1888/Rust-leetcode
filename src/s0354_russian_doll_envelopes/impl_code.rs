pub struct Solution;

impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        if envelopes.is_empty() {
            return 0;
        }
        envelopes.sort_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        let heights: Vec<i32> = envelopes.iter().map(|e| e[1]).collect();
        Self::length_of_lis(heights)
    }

    fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut d = vec![0; nums.len()];
        let mut length_lis = 0;
        for val in nums {
            let index = Self::binary_search_ins_pos(&d, length_lis, val);
            d[index] = val;
            if index == length_lis {
                length_lis += 1;
            }
        }
        length_lis as i32
    }

    fn binary_search_ins_pos(d: &[i32], end: usize, target: i32) -> usize {
        let mut start = 0usize;
        let mut end = end;
        while start < end {
            let mid = start + (end - start) / 2;
            if d[mid] >= target {
                end = mid;
            } else {
                start = mid + 1;
            }
        }
        start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_envelopes() {
        let envelopes = vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]];
        assert_eq!(Solution::max_envelopes(envelopes), 3);

        let envelopes2 = vec![vec![4, 5], vec![4, 6], vec![6, 7], vec![2, 3], vec![1, 1]];
        assert_eq!(Solution::max_envelopes(envelopes2), 4);
    }
}
