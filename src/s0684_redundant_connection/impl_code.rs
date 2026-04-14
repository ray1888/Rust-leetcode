pub struct Solution;

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut parent: Vec<usize> = (0..=n).collect();
        let mut rank = vec![0; n + 1];

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            if Self::find(&mut parent, u) == Self::find(&mut parent, v) {
                return edge;
            }
            Self::union(&mut parent, &mut rank, u, v);
        }
        vec![]
    }

    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = Self::find(parent, parent[x]);
        }
        parent[x]
    }

    fn union(parent: &mut Vec<usize>, rank: &mut Vec<i32>, x: usize, y: usize) {
        let root_x = Self::find(parent, x);
        let root_y = Self::find(parent, y);
        if root_x == root_y {
            return;
        }
        if rank[root_x] > rank[root_y] {
            parent[root_y] = root_x;
        } else if rank[root_x] < rank[root_y] {
            parent[root_x] = root_y;
        } else {
            parent[root_y] = root_x;
            rank[root_x] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_redundant_connection() {
        let cases = vec![
            (vec![vec![1, 2], vec![1, 3], vec![2, 3]], vec![2, 3]),
            (
                vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]],
                vec![1, 4],
            ),
        ];

        for (edges, expected) in cases {
            let actual = Solution::find_redundant_connection(edges);
            assert_eq!(actual, expected);
        }
    }
}
