pub struct Solution;

use std::cmp::max;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut dp_i0 = 0;
        let mut dp_i1 = i32::MIN;
        let mut dp_pre0 = 0;
        for price in prices {
            let tmp = dp_i0;
            dp_i0 = max(dp_i0, dp_i1 + price);
            dp_i1 = max(dp_i1, dp_pre0 - price);
            dp_pre0 = tmp;
        }
        dp_i0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
    }
}
