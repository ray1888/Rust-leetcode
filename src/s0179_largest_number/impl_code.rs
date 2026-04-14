pub struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        if nums.is_empty() {
            return String::new();
        }
        let mut strs: Vec<String> = nums.iter().map(|x| x.to_string()).collect();
        strs.sort_by(|a, b| {
            let o1 = format!("{}{}", a, b);
            let o2 = format!("{}{}", b, a);
            o2.cmp(&o1)
        });
        if strs[0] == "0" {
            return "0".to_string();
        }
        strs.concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_number() {
        assert_eq!(Solution::largest_number(vec![3, 30, 34, 5, 9]), "9534330");
    }
}
