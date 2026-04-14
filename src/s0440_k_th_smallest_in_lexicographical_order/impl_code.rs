pub struct Solution;

impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let mut pre_num = 1;
        let mut parent = 1;
        while pre_num < k {
            let count = Self::get_count(parent, n);
            if count + pre_num > k {
                parent *= 10;
                pre_num += 1;
            } else {
                parent += 1;
                pre_num += count;
            }
        }
        parent
    }

    fn get_count(prefix: i32, n: i32) -> i32 {
        let mut cur = prefix as i64;
        let mut next = (prefix + 1) as i64;
        let n = n as i64;
        let mut count = 0i64;
        while cur <= n {
            count += next.min(n + 1) - cur;
            cur *= 10;
            next *= 10;
        }
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kth_number() {
        assert_eq!(Solution::find_kth_number(13, 2), 10);
    }
}
