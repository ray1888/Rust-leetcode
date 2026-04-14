pub struct Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        #[inline(always)]
        fn is_palindrome(bs: &[u8], mut i: usize, mut j: usize) -> bool {
            while j > i {
                if unsafe { bs.get_unchecked(i) == bs.get_unchecked(j) } {
                    i += 1;
                    j -= 1;
                } else {
                    return false;
                }
            }
            true
        }
        let bs = s.into_bytes();
        let (mut i, mut j) = (0usize, bs.len().saturating_sub(1));
        while j > i {
            if unsafe { bs.get_unchecked(i) == bs.get_unchecked(j) } {
                i += 1;
                j -= 1;
            } else {
                return is_palindrome(&bs, i, j - 1) || is_palindrome(&bs, i + 1, j);
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_palindrome() {
        assert_eq!(Solution::valid_palindrome("aba".to_string()), true);
    }
}
