pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 0 {
            return vec!["".to_string()];
        } else if n == 1 {
            return vec!["()".to_string()];
        }
        let mut ans = vec![];
        for i in 0..n {
            let left = Self::generate_parenthesis(i);
            let right = Self::generate_parenthesis(n - i - 1);
            for l in left.iter() {
                for r in right.iter() {
                    ans.push(format!("({}){}", l, r));
                }
            }
        }
        return ans;
    }
}

// TODO: @cjh,to find way to concat string
// impl Solution {
//     pub fn generate_parenthesis(n: i32) -> Vec<String> {
//         if n == 0 {
//             return vec![];
//         }

//         let mut result :Vec<String> = vec!["".to_string(); (n+1) as usize];
//         for i in 1..n{
//             for j in 0..i{
//                 for (_,sLeft) in result[j as usize].bytes().enumerate(){
//                     for (_, sRight) in  result[(i-j-1) as usize].bytes().enumerate() {
//                         let new = format!("{}{}{}{}", "(", sLeft, ")", sRight);
//                         result[i as usize].push(new.chars());
//                     }
//                 }
//             }
//         }

//         return result;
//     }
// }
