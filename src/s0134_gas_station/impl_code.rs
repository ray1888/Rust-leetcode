pub struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        for i in 0..n {
            let mut tank = 0;
            for j in i..i + n {
                let k = j % n;
                tank += gas[k] - cost[k];
                if tank < 0 {
                    break;
                }
            }
            if tank >= 0 {
                return i as i32;
            }
        }
        -1
    }

    pub fn can_complete_circuit_skip(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut i = 0;
        let mut j;
        while i < n {
            let mut tank = 0;
            j = i;
            while j < i + n {
                let k = j % n;
                tank += gas[k] - cost[k];
                if tank < 0 {
                    break;
                }
                j += 1;
            }
            if tank >= 0 {
                return i as i32;
            }
            i = j + 1;
        }
        -1
    }

    pub fn can_complete_circuit_greedy(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut start = 0;
        let mut total = 0;
        let mut tank = 0;
        for i in 0..n {
            total += gas[i] - cost[i];
            tank += gas[i] - cost[i];
            if tank < 0 {
                start = i + 1;
                tank = 0;
            }
        }
        if total < 0 {
            -1
        } else {
            start as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_complete_circuit() {
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 2, 4, 8], vec![4, 8, 1, 2]),
            2
        );
    }

    #[test]
    fn test_can_complete_circuit_skip() {
        assert_eq!(
            Solution::can_complete_circuit_skip(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
        assert_eq!(
            Solution::can_complete_circuit_skip(vec![1, 2, 4, 8], vec![4, 8, 1, 2]),
            2
        );
    }

    #[test]
    fn test_can_complete_circuit_greedy() {
        assert_eq!(
            Solution::can_complete_circuit_greedy(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
        assert_eq!(
            Solution::can_complete_circuit_greedy(vec![1, 2, 4, 8], vec![4, 8, 1, 2]),
            2
        );
    }
}
