pub struct Solution;


impl Solution {
    pub fn longest_palindrome(s: String) -> String {
     let (mut start, mut end) = (0, 0);
        let s = s.chars().collect::<Vec<_>>();
        let mut dp = vec![vec![false; s.len()]; s.len()];
        let mut maxLength = 0;
        for i in (0..s.len()).rev(){
            for j in i..s.len(){
                if i == j {
                    dp[i][j] = true;
                } else if i+1 == j {
                    dp[i][j] = s[i] == s[j];
                } else{
                    dp[i][j] = s[i] == s[j] && dp[i+1][j-1];
                }
                if dp[i][j] && (j-i+1 > maxLength){
                    start = i ;
                    maxLength = j- i +1;
                }
            }
        }
        return s[start..start+maxLength].iter().collect()

    }
}