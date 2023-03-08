pub struct Solution;


use std::collections::{HashMap, VecDeque};
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut income = vec![0; num_courses as usize];
        let mut map = HashMap::new();
        prerequisites.iter().for_each(|x| {
            let (a, b) = (x[0] as usize, x[1] as usize);
            income[a] += 1;
            map.entry(b).or_insert(vec![]).push(a);
        });
        let mut courses = VecDeque::new();
        for (i, &n) in income.iter().enumerate() {
            if n == 0 {
                courses.push_back(i);
            }
        }

        let mut ans = vec![];
        while !courses.is_empty() {
            let course = courses.pop_front().unwrap();
            ans.push(course as i32);
            if let Some(next) = map.get(&course) {
                for &n in next {
                    income[n] -= 1;
                    if income[n] == 0 {
                        courses.push_back(n);
                    }
                }
            }
        }
        if ans.len() == num_courses as usize {
            ans
        } else {
            vec![]
        }
    }
}
