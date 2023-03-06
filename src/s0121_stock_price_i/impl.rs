pub struct Solution;


use std::cmp::max;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_value = prices[0];
        let mut max_profit = 0;

        for i in 1..prices.len(){
            if min_value > prices[i]{
                min_value = prices[i];
            }
            max_profit = max(prices[i]-min_value, max_profit);
        }
        return max_profit;
    }
}