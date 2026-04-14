pub struct Solution;

use std::cmp::max;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut min_val = prices[0];
        let mut cur = 0;
        for price in prices {
            if price < min_val {
                min_val = price;
            }
            cur = max(cur, price - min_val);
        }
        cur
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
