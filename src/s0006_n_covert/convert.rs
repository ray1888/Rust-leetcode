pub struct Solution;


impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        let mut rows = vec![String::new();num_rows];
        let time: usize = (s.len()/num_rows)+1;
        let mut iter: Vec<usize> = (0..num_rows).chain((1..num_rows-1).rev()).collect();

        let temp = iter.clone();
        for i in 0..time {
            iter.extend(&temp);
        }
        iter.into_iter().zip(s.chars()).for_each(|(i,c)| rows[i].push(c));
        rows.into_iter().collect()
    }
}


// impl Solution {
//     pub fn convert(s: String, num_rows: i32) -> String {
//        if num_rows == 1 {
//             return s;
//         }
//         let string_bytes = s.as_bytes();
//         let length = s.len();
//         let period = num_rows * 2 -2;
//         let mut result :Vec<u8>= vec![length;0];
//         for i in 0..length{
//             let modd = i % period;
//             if modd < num_rows{
//                 result[modd] = string_bytes[i];
//             } else {
//                 result[period-modd] = string_bytes[i];
//             }
//         }
//         match String::from_utf8(result){
//             Ok(a) => a,
//             Err(_) => "",
//         }
//     }
// }
