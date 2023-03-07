pub struct Solution;


impl Solution {
    pub fn count_substrings(s: String) -> i32 {
     let (mut start, mut end) = (0, 0);
        let s = s.chars().collect::<Vec<_>>();
        let mut dp = vec![vec![false; s.len()]; s.len()];
        let mut counter = 0;
        for i in (0..s.len()).rev(){
            for j in i..s.len(){
                if i == j {
                    dp[i][j] = true;
                } else if i+1 == j {
                    dp[i][j] = s[i] == s[j];
                } else{
                    dp[i][j] = s[i] == s[j] && dp[i+1][j-1];
                }
                if dp[i][j] {
                    counter+=1;
                }
            }
        }
        return counter
    }
}