pub struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest_partition(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        if nums.is_empty() || k > nums.len() {
            return -1;
        }
        let mut low = 0usize;
        let mut high = nums.len() - 1;
        while low <= high {
            let p = Self::partition(&mut nums, low, high);
            if p == k - 1 {
                return nums[p];
            } else if p > k - 1 {
                high = p - 1;
            } else {
                low = p + 1;
            }
        }
        -1
    }

    fn partition(nums: &mut [i32], start: usize, end: usize) -> usize {
        let mut i = start;
        let mut j = end;
        let pivot = nums[end];
        while i < j {
            while nums[i] >= pivot && i < j {
                i += 1;
            }
            if i < j {
                nums.swap(i, j);
            }
            while nums[j] < pivot && i < j {
                j -= 1;
            }
            if i < j {
                nums.swap(i, j);
            }
        }
        i
    }

    pub fn find_kth_largest_min_heap(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        if nums.is_empty() || k > nums.len() {
            return -1;
        }
        let mut pq: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        for num in nums {
            if pq.len() >= k {
                if let Some(&Reverse(top)) = pq.peek() {
                    if top < num {
                        pq.pop();
                        pq.push(Reverse(num));
                    }
                }
            } else {
                pq.push(Reverse(num));
            }
        }
        match pq.peek() {
            Some(&Reverse(v)) => v,
            None => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_kth_largest_partition() {
        let cases = vec![
            (vec![2, 1, 5, 6], 1, 6),
            (vec![1, 5, 4], 3, 1),
            (vec![1, 9, 5, 3, 2, 6], 3, 5),
            (vec![], 3, -1),
            (vec![1, 9, 5, 3, 2, 6], 7, -1),
        ];
        for (input, k, want) in cases {
            let actual = Solution::find_kth_largest_partition(input, k);
            assert_eq!(actual, want, "find_kth_largest_partition failed");
        }
    }

    #[test]
    fn test_find_kth_largest_min_heap() {
        let cases = vec![
            (vec![2, 1, 5, 6], 1, 6),
            (vec![1, 5, 4], 3, 1),
            (vec![1, 9, 5, 3, 2, 6], 3, 5),
            (vec![], 3, -1),
            (vec![1, 9, 5, 3, 2, 6], 7, -1),
        ];
        for (input, k, want) in cases {
            let actual = Solution::find_kth_largest_min_heap(input, k);
            assert_eq!(actual, want, "find_kth_largest_min_heap failed");
        }
    }
}
