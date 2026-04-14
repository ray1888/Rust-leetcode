pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let length = heights.len();
        let mut stack: Vec<usize> = vec![];
        let mut max_area = 0;
        for i in 0..=length {
            let h = if i < length { heights[i] } else { 0 };
            while !stack.is_empty() && h < heights[*stack.last().unwrap()] {
                let idx = stack.pop().unwrap();
                let l = if let Some(&top) = stack.last() {
                    top as i32
                } else {
                    -1
                };
                let area = (i as i32 - l - 1) * heights[idx];
                max_area = max_area.max(area);
            }
            stack.push(i);
        }
        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_rectangle_area() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
    }
}
