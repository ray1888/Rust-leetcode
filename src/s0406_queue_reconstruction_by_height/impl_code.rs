pub struct Solution;

impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;
        people.sort_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });

        let n = people.len();
        let mut ans: Vec<Vec<i32>> = vec![Vec::new(); n];
        for person in people {
            let mut spaces = person[1] + 1;
            for i in 0..n {
                if ans[i].is_empty() {
                    spaces -= 1;
                    if spaces == 0 {
                        ans[i] = person;
                        break;
                    }
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reconstruct_queue() {
        let people = vec![
            vec![7, 0],
            vec![4, 4],
            vec![7, 1],
            vec![5, 0],
            vec![6, 1],
            vec![5, 2],
        ];
        let expected = vec![
            vec![5, 0],
            vec![7, 0],
            vec![5, 2],
            vec![6, 1],
            vec![4, 4],
            vec![7, 1],
        ];
        assert_eq!(Solution::reconstruct_queue(people), expected);
    }
}
