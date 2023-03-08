pub struct Solution;

impl Solution {
    fn partition_string(s:&str, start: usize,dp: &Vec<Vec<bool>>,
        result: &mut Vec<Vec<String>>, elem:&mut Vec<String>) {
        if start == s.len(){
            result.push(elem.to_vec());
        }else{
            for end in start..s.len(){
                if dp[start][end]{
                    elem.push(String::from(&s[start..=end]));
                    Self::partition_string(s,end+1,dp,result,elem);
                    elem.pop();
                }
            }
        }
    }

    pub fn partition(s: String) -> Vec<Vec<String>> {
        if s.len() == 0 {
            return vec![];
        }
        let mut result = vec![];
        let n = s.len();
        let mut dp :Vec<Vec<bool>> = vec![vec![false;n];n];
        for i in (0..s.len()).rev(){
            for j in i..s.len(){
                if i == j {
                    dp[i][j] = true 
                } else if (i+1) as usize == j {
                    dp[i][j] = s.chars().nth(i) == s.chars().nth(j);
                } else {
                    dp[i][j] = s.chars().nth(i) == s.chars().nth(j) && dp[(i+1) as usize][(j-1) as usize];
                }
            }
        }
        let mut elem :Vec<String> = vec![];
        Self::partition_string(&s[..],0,&dp, &mut result, &mut elem);
        result
    }
}