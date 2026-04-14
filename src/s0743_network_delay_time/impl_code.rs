pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut graph = vec![vec![-1; n + 1]; n + 1];
        let mut dist = vec![i32::MAX; n + 1];
        dist[k] = 0;
        for item in times.iter() {
            graph[item[0] as usize][item[1] as usize] = item[2];
        }
        let mut st = vec![false; n + 1];
        for _ in 1..=n {
            let index = Self::min_distance_index(&st, &dist);
            st[index] = true;
            for item in 1..=n {
                if !st[item]
                    && graph[index][item] != -1
                    && dist[index] != i32::MAX
                    && dist[item] > dist[index] + graph[index][item]
                {
                    dist[item] = dist[index] + graph[index][item];
                }
            }
        }
        let mut dist = dist[1..].to_vec();
        dist.sort_unstable();
        if dist[n - 1] == i32::MAX {
            -1
        } else {
            dist[n - 1]
        }
    }

    fn min_distance_index(st: &[bool], dist: &[i32]) -> usize {
        let mut min_index = 0usize;
        let mut min_val = i32::MAX;
        for i in 1..dist.len() {
            if !st[i] && dist[i] <= min_val {
                min_index = i;
                min_val = dist[i];
            }
        }
        min_index
    }

    pub fn bellman_ford(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut dist = vec![i32::MAX; n + 1];
        dist[k] = 0;
        for _ in 0..n - 1 {
            for item in times.iter() {
                let u = item[0] as usize;
                let v = item[1] as usize;
                let weight = item[2];
                if dist[u] != i32::MAX && dist[v] > dist[u] + weight {
                    dist[v] = dist[u] + weight;
                }
            }
        }
        for item in times.iter() {
            let u = item[0] as usize;
            let v = item[1] as usize;
            let weight = item[2];
            if dist[u] != i32::MAX && dist[v] > dist[u] + weight {
                return -1;
            }
        }
        let mut dist = dist[1..].to_vec();
        dist.sort_unstable();
        if dist[n - 1] == i32::MAX {
            -1
        } else {
            dist[n - 1]
        }
    }

    pub fn spfa_algorithm(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut graph = vec![vec![i32::MAX; n + 1]; n + 1];
        let mut dist = vec![i32::MAX; n + 1];
        let mut working_queue = VecDeque::new();
        let mut in_queue = vec![false; n + 1];
        for item in times.iter() {
            graph[item[0] as usize][item[1] as usize] = item[2];
        }
        dist[k] = 0;
        in_queue[k] = true;
        working_queue.push_back(k);
        while let Some(u) = working_queue.pop_front() {
            in_queue[u] = false;
            for v in 1..=n {
                if v == u {
                    continue;
                }
                if graph[u][v] != i32::MAX && dist[v] > dist[u] + graph[u][v] {
                    dist[v] = dist[u] + graph[u][v];
                    if !in_queue[v] {
                        working_queue.push_back(v);
                        in_queue[v] = true;
                    }
                }
            }
        }
        let mut dist = dist[1..].to_vec();
        dist.sort_unstable();
        if dist[n - 1] == i32::MAX {
            -1
        } else {
            dist[n - 1]
        }
    }

    pub fn floyd_warshall_algorithm(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut dist = vec![vec![i32::MAX; n + 1]; n + 1];
        for item in times.iter() {
            dist[item[0] as usize][item[1] as usize] = item[2];
        }
        for k_i in 1..=n {
            for i in 1..=n {
                for j in 1..=n {
                    if dist[i][k_i] != i32::MAX
                        && dist[k_i][j] != i32::MAX
                        && dist[i][j] > dist[i][k_i] + dist[k_i][j]
                    {
                        dist[i][j] = dist[i][k_i] + dist[k_i][j];
                    }
                }
            }
        }
        let mut row = dist[k].clone();
        row.sort_unstable();
        let mut max_value = 0;
        for item in row.iter() {
            if *item != i32::MAX && *item > max_value {
                max_value = *item;
            }
        }
        max_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normal_times() -> Vec<Vec<i32>> {
        vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]]
    }

    fn extra_long_times() -> Vec<Vec<i32>> {
        vec![
            vec![3, 5, 78],
            vec![2, 1, 1],
            vec![1, 3, 0],
            vec![4, 3, 59],
            vec![5, 3, 85],
            vec![5, 2, 22],
            vec![2, 4, 23],
            vec![1, 4, 43],
            vec![4, 5, 75],
            vec![5, 1, 15],
            vec![1, 5, 91],
            vec![4, 1, 16],
            vec![3, 2, 98],
            vec![3, 4, 22],
            vec![5, 4, 31],
            vec![1, 2, 0],
            vec![2, 5, 4],
            vec![4, 2, 51],
            vec![3, 1, 36],
            vec![2, 3, 59],
        ]
    }

    #[test]
    fn test_network_delay_time() {
        assert_eq!(Solution::network_delay_time(normal_times(), 4, 2), 2);
        assert_eq!(Solution::network_delay_time(extra_long_times(), 5, 5), 31);
    }

    #[test]
    fn test_bellman_ford() {
        assert_eq!(Solution::bellman_ford(normal_times(), 4, 2), 2);
        assert_eq!(Solution::bellman_ford(extra_long_times(), 5, 5), 31);
    }

    #[test]
    fn test_spfa_algorithm() {
        assert_eq!(Solution::spfa_algorithm(normal_times(), 4, 2), 2);
        assert_eq!(Solution::spfa_algorithm(extra_long_times(), 5, 5), 31);
    }

    #[test]
    fn test_floyd_warshall_algorithm() {
        assert_eq!(Solution::floyd_warshall_algorithm(normal_times(), 4, 2), 2);
        assert_eq!(
            Solution::floyd_warshall_algorithm(extra_long_times(), 5, 5),
            31
        );
    }
}
