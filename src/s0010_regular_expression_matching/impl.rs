pub struct Solution;

impl Solution {
    fn is_equal(s: u8, p: u8) ->bool {
        s == p || p == '.' as u8
    }

    pub fn is_match(s: String, p: String) -> bool {
        let length_s = s.len();
        let length_p = p.len();
        let mut p = p.as_bytes();
        let mut s = s.as_bytes();

        let mut dp = vec![vec![false;length_p+1];length_s+1];

        dp[0][0] = true;

        for i in 1..length_p+1{
            if p[i-1] == '*' as u8 {
                dp[0][i] = dp[0][i-2];
            } else {
                dp[0][i] = false;
            }
        }

        for i in 1..length_s+1{
            for j in 1..length_p+1{
                let sc = s[i-1];
                let pc = p[j-1];
                if Solution::is_equal(sc,pc) {
                    dp[i][j] = dp[i-1][j-1];
                } else if pc == '*' as u8 {
                    let preChar = p[j-2];
                    if Solution::is_equal(sc,preChar){
                        // 这个是因为如果为*后对比的是0.。si，0.。pprechar
                        dp[i][j] = dp[i-1][j] || dp[i][j-1] || dp[i][j-2]
                    } else {
                        dp[i][j] = dp[i][j-2]
                    }
                } else {
                    dp[i][j] = false 
                }

            }
        }


        return dp[length_s][length_p];
    }
}