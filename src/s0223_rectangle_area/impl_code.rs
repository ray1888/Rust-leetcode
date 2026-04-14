pub struct Solution;

impl Solution {
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        let w = (ax2.min(bx2) - ax1.max(bx1)).max(0);
        let h = (ay2.min(by2) - ay1.max(by1)).max(0);
        (ax2 - ax1) * (ay2 - ay1) - w * h + (bx2 - bx1) * (by2 - by1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_area() {
        assert_eq!(Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
    }
}
