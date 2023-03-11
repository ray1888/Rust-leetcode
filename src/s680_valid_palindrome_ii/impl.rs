pub struct Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let mut low = 0;
        let mut high = s.len()-1;

        while low < high {
            if s.chars().nth(i).unwrap() == s.chars().nth(j).unwrap(){
                low+=1;
                high-=1;
            } else {
                let mut flag1 = true;
                let mut flag2 = true;
                let mut i = low;
                let mut j = high-1;
                while i < j {
                    if s.chars().nth(i).unwrap() != s.chars().nth(j).unwrap(){
                        flag1 = false;
                        break;
                    }
                    i+=1;
                    j-=1;
                }
                i = low +1;
                j = high;
                while i < j {
                    if s.chars().nth(i).unwrap() != s.chars().nth(j).unwrap(){
                        flag2 = false;
                        break;
                    }
                    i+=1;
                    j-=1;
                }
                return flag1 || flag2
            }
        }
        return true;
    }
}

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        #[inline(always)]
        fn is(s: &Vec<u8>, mut i: usize, mut j: usize) -> bool {
            while j > i {
                if unsafe { s.get_unchecked(i) == s.get_unchecked(j) } {
                    i += 1;
                    j -= 1;
                } else {
                    return false;
                }
            }
            true
        }
        let (mut i, mut j, bs) = (0usize, s.len() - 1, s.into_bytes());
        while j > i {
            if unsafe { bs.get_unchecked(i) == bs.get_unchecked(j) } {
                i += 1;
                j -= 1;
            } else {
                return is(&bs, i, j - 1) || is(&bs, i + 1, j);
            }
        }
        true
    }
}
