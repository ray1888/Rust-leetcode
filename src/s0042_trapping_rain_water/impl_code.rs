pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        Self::trap_space_o1(&height)
    }

    pub fn trap_space_on(height: &[i32]) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let mut left_max = -1;
        let mut right_max = -1;
        let mut water = 0;
        let length = height.len();
        let mut d = vec![0; length];
        for i in 0..length {
            left_max = left_max.max(height[i]);
            d[i] = left_max;
        }
        for i in (0..length).rev() {
            right_max = right_max.max(height[i]);
            d[i] = d[i].min(right_max);
            water += d[i] - height[i];
        }
        water
    }

    pub fn trap_space_o1(height: &[i32]) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let mut left_max = -1;
        let mut right_max = -1;
        let mut water = 0;
        let length = height.len();
        let mut i = 0;
        let mut j = length - 1;
        while i < j {
            left_max = left_max.max(height[i]);
            right_max = right_max.max(height[j]);
            if left_max < right_max {
                water += left_max - height[i];
                i += 1;
            } else {
                water += right_max - height[j];
                j -= 1;
            }
        }
        water
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap_space_on() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(Solution::trap_space_on(&height), 6);
    }

    #[test]
    fn test_trap_space_o1() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(Solution::trap_space_o1(&height), 6);
    }
}
