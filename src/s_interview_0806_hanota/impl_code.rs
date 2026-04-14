pub struct Solution;

impl Solution {
    pub fn hanota(a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) {
        if a.is_empty() {
            return;
        }
        let n = a.len();
        let mut a_clone = a.clone();
        let mut b_clone = b.clone();
        let mut c_clone = c.clone();
        Self::move_disks(n, &mut a_clone, &mut b_clone, &mut c_clone);
        a.clear();
        b.clear();
        c.extend(c_clone);
    }

    fn move_disks(n: usize, a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) {
        if n == 0 {
            return;
        }
        if n == 1 {
            c.push(a.pop().unwrap());
        } else {
            Self::move_disks(n - 1, a, c, b);
            Self::move_disks(1, a, b, c);
            Self::move_disks(n - 1, b, a, c);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_hanota() {
        let mut a = vec![2, 1, 0];
        let mut b = vec![];
        let mut c = vec![];
        Solution::hanota(&mut a, &mut b, &mut c);
        assert_eq!(c, vec![2, 1, 0]);
    }
}
