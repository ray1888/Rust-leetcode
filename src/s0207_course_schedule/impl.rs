pub struct Solution;


use std::collections::HashMap;

// impl Solution {
//     fn has_cycle(graph: HashMap<i32,Vec<i32>>, check:HashMap<i32,bool>, visited:HashMap<i32,bool>, index:usize) -> bool {
//         if visited.get(&index) {
//             return true 
//         }
//         visited[index] = true;
//         for i in graph[index].len() {
//             if !check_map.get(num_courses[i]) && Self::has_cycle(graph,check_map,visited_map,i){
//                 return true 
//             }
//         }
//         visited[index] = false;
//         check[index] = true;
//         return false
//     }

//     pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
//         if num_courses == 0 {
//             return true 
//         }

//         let mut check_map :HashMap<i32,bool> = HashMap::new();
//         let mut visited_map :HashMap<i32,bool> = HashMap::new();
//         let mut graph :HashMap<i32,Vec<i32>> = HashMap::new();
//         for i in 0..prerequisites.len(){
//             let before = prerequisites[i][1];
//             let after = prerequisites[i][0];
//             check_map[before] = false;
//             visited_map[before] = false;
//             graph[before].push(after);
//         }

//         let mut visited :Vec<Vec<bool> = vec![vec![false;num_courses];num_courses];
//         for i in 0..num_courses{
//             if !check_map.get(num_courses[i]) && Self::has_cycle(graph,check_map,visited_map,i) {
//                 return false; 
//             }
//         }
//         return true
//     }   
// }


impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let graph = Solution::build_graph(num_courses, prerequisites);
        let mut has_cycle = false;

        let (mut visited, mut on_path): (Vec<bool>, Vec<bool>) =
            (vec![false; num_courses], vec![false; num_courses]);

        for s in 0..num_courses {
            let s = s as usize;
            Solution::traverse(&graph, s, &mut visited, &mut on_path, &mut has_cycle);
        }

        !has_cycle
    }

    pub fn traverse(
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
            let v = v;
            Solution::traverse(graph, v, visited, on_path, has_cycle);
        }

        on_path[s] = false;
    }

    pub fn build_graph(num_courses: usize, prerequisites: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
        let mut graph = vec![vec![]; num_courses];

        for edge in prerequisites.iter() {
            let (from, to) = (edge[1] as usize, edge[0] as usize);
            graph[from].push(to);
        }

        graph
    }
}
