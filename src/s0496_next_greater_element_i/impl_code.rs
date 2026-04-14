use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        if nums1.is_empty() || nums2.is_empty() {
            return vec![];
        }
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut stack: Vec<i32> = vec![];
        for v in nums2 {
            while !stack.is_empty() && v > *stack.last().unwrap() {
                map.insert(stack.pop().unwrap(), v);
            }
            stack.push(v);
        }
        let mut res = vec![];
        for v in nums1 {
            if let Some(&val) = map.get(&v) {
                res.push(val);
            } else {
                res.push(-1);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_greater_element() {
        assert_eq!(
            Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
            vec![-1, 3, -1]
        );
    }
}
