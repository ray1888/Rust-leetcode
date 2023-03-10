pub struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut res : f64 = 1.0;
        let mut m = n;
        let mut xx = x;
        loop {
            if m == 0 {
                break;
            }
            if m %2 != 0{
                res *= xx;
            }
            xx *=xx;
            m /= 2;
        }
        if n < 0 {
            return 1.0 / res;
        }else {
            return res;
        }
    }
}