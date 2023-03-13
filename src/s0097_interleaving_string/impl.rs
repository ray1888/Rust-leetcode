pub struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len(){
            return false;
        }
        let l1 = s1.len();
        let l2 = s2.len();
        let l3 = s3.len();
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();

        // 存放的是s3(0..i-1) 是否可以有 s1(0..i-1) 和 s2(0..i-1)
        let mut dp = vec![vec![false;l1+1];l2+1];
        dp[0][0] = true;

        // 状态转移方程开始计算，可以从头或者尾来计算，因为头或者尾必须跟s1,或者s2的字符相等

        for i in 1..l1+1{
            dp[0][i] = dp[0][i-1] && s1[i-1] == s3[i-1];
        }

        for j in 1..l2+1{
            dp[j][0] = dp[j-1][0] && s2[j-1] == s3[j-1];
        }

        for i in 1..l2+1{
            for j in 1..l1+1{
                let k = i + j;
                dp[i][j] = (dp[i-1][j] &&  s2[i-1] == s3[k-1]) || (dp[i][j-1] &&  s1[j-1] == s3[k-1])
            }
        }
        return dp[l2][l1];
    }
}