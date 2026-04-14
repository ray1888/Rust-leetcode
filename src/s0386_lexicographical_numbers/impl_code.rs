pub struct Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        Self::dfs(0, n, &mut ans);
        ans
    }

    fn dfs(m: i32, n: i32, ans: &mut Vec<i32>) {
        if m > n {
            return;
        }
        for i in 0..10 {
            let val = m + i;
            if val <= n && val != 0 {
                ans.push(val);
                Self::dfs(val * 10, n, ans);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexical_order() {
        assert_eq!(
            Solution::lexical_order(13),
            vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }
}
