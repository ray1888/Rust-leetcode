impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || num_rows == s.len() as i32 {
            return s;
        }
        let s = s.chars().collect::<Vec<_>>();
        let t = num_rows as usize * 2 - 2;
        let mut ans = vec![];
        for i in (0..s.len()).step_by(t) {
            ans.push(s[i]);
        }

        for start in 1..num_rows as usize - 1 {
            for i in (start..s.len()).step_by(t) {
                ans.push(s[i]);
                if i + t - 2 * start < s.len() {
                    ans.push(s[i + t - 2 * start]);
                }
            }
        }
        for i in (num_rows as usize - 1..s.len()).step_by(t) {
            ans.push(s[i]);
        }
        ans.iter().collect()
    }
}

// impl Solution {
//     pub fn convert(s: String, num_rows: i32) -> String {
//         let num_rows = num_rows as usize;
//         let mut arr = vec![String::new(); num_rows];
//         let iter = (0..num_rows).chain((1..num_rows - 1).rev()).cycle();
//         iter.zip(s.chars()).for_each(|(i, c)| arr[i].push(c));
//         arr.into_iter().collect()
//     }
// }
