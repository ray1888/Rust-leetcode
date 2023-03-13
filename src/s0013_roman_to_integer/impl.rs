pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let mut m = std::collections::HashMap::new();
        m.insert('I', 1);
        m.insert('V', 5);
        m.insert('X',10);
        m.insert('L', 50);
        m.insert('C', 100);
        m.insert('D', 500);
        m.insert('M', 1000);


        let length = s.len();
        let mut result:i32 = *m.get(&s.chars().nth(length-1).unwrap()).unwrap();
        for i in (0..length-1).rev(){
            let curr = m.get(&s.chars().nth(i).unwrap()).unwrap();
            let next = m.get(&s.chars().nth(i+1).unwrap()).unwrap();
            if curr < next{
                result -= curr;
            } else {
                result += curr;
            }
        }
        return result
    }
}