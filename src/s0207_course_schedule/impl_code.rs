pub struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let graph = Solution::build_graph(num_courses, prerequisites);
        let mut has_cycle = false;

        let (mut visited, mut on_path): (Vec<bool>, Vec<bool>) =
            (vec![false; num_courses], vec![false; num_courses]);

        for s in 0..num_courses {
            Solution::traverse(&graph, s, &mut visited, &mut on_path, &mut has_cycle);
        }

        !has_cycle
    }

    fn traverse(
        graph: &Vec<Vec<usize>>,
        s: usize,
        visited: &mut Vec<bool>,
        on_path: &mut Vec<bool>,
        has_cycle: &mut bool,
    ) {
        if on_path[s] {
            *has_cycle = true;
        }

        if visited[s] || *has_cycle {
            return;
        }

        visited[s] = true;
        on_path[s] = true;

        for &v in graph[s].iter() {
            Solution::traverse(graph, v, visited, on_path, has_cycle);
        }

        on_path[s] = false;
    }

    fn build_graph(num_courses: usize, prerequisites: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
        let mut graph = vec![vec![]; num_courses];

        for edge in prerequisites.iter() {
            let (from, to) = (edge[1] as usize, edge[0] as usize);
            graph[from].push(to);
        }

        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_course_schedule_dfs() {
        let tests = vec![
            (2, vec![vec![1, 0], vec![0, 1]], false),
            (2, vec![vec![0, 1]], true),
        ];

        for (num_courses, prerequisites, expected) in tests {
            assert_eq!(Solution::can_finish(num_courses, prerequisites), expected);
        }
    }
}
