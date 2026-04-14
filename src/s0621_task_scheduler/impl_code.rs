use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut counts = [0; 26];
        for task in tasks {
            counts[task as usize - 'A' as usize] += 1;
        }

        let mut pq = BinaryHeap::new();
        for count in counts {
            if count > 0 {
                pq.push(count);
            }
        }

        let mut result = 0;
        let mut idle = 0;
        while !pq.is_empty() {
            result += n + 1;
            idle = n + 1 - pq.len() as i32;
            let size = std::cmp::min((n + 1) as usize, pq.len());
            let mut temp = Vec::with_capacity(size);
            for _ in 0..size {
                if let Some(count) = pq.pop() {
                    temp.push(count - 1);
                }
            }
            for count in temp {
                if count > 0 {
                    pq.push(count);
                }
            }
        }

        result - idle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_least_interval() {
        assert_eq!(
            Solution::least_interval(
                vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'],
                2
            ),
            16
        );
    }
}
