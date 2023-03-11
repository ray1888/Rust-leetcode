pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
       
        let list: Vec<char> = s.to_lowercase().chars().collect();
        let mut left = 0;
        let mut right = list.len() -1 ;
        

        while left < right{
            let mut v1 = list.get(left).unwrap();
            let mut v2 = list.get(right).unwrap();

            while left < right && !v1.is_alphanumeric()  {
                left+=1;
                v1 = list.get(left).unwrap();
            }
            while left < right && !v2.is_alphanumeric() {
                right-=1;
                v2 = list.get(right).unwrap();
            }

            if left < right{
                if v1 != v2 {
                    return false;
                }
                left+=1;
                right-=1;
            }
        }
        return true;
    }
}