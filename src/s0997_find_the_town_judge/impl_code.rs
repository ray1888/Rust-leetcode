pub struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut in_degree = vec![0; (n + 1) as usize];
        let mut out_degree = vec![0; (n + 1) as usize];

        for t in trust.iter() {
            out_degree[t[0] as usize] += 1;
            in_degree[t[1] as usize] += 1;
        }

        for i in 1..=n {
            if in_degree[i as usize] == n - 1 && out_degree[i as usize] == 0 {
                return i;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_judge() {
        let cases = vec![
            (2, vec![vec![1, 2]], 2),
            (3, vec![vec![1, 3], vec![2, 3]], 3),
            (3, vec![vec![1, 3], vec![2, 3], vec![3, 1]], -1),
        ];

        for (n, trust, expected) in cases {
            let actual = Solution::find_judge(n, trust);
            assert_eq!(actual, expected);
        }
    }
}
