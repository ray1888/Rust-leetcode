use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        if tickets.is_empty() {
            return vec![];
        }
        let mut route_map: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for ticket in tickets.iter() {
            route_map
                .entry(ticket[0].clone())
                .or_default()
                .push(ticket[1].clone());
        }
        for destinations in route_map.values_mut() {
            destinations.sort();
        }
        let total = tickets.len() + 1;
        let mut result = vec!["JFK".to_string()];
        let start = "JFK".to_string();
        Self::dfs(&start, total, &route_map, &mut result);
        result
    }

    fn dfs(
        start: &String,
        total: usize,
        route_map: &BTreeMap<String, Vec<String>>,
        result: &mut Vec<String>,
    ) -> bool {
        if result.len() == total {
            return true;
        }
        let destinations = match route_map.get(start) {
            Some(d) => d,
            None => return false,
        };
        for i in 0..destinations.len() {
            let dest = &destinations[i];
            if Self::is_visited(start, i, route_map, result) {
                continue;
            }
            result.push(dest.clone());
            if Self::dfs(dest, total, route_map, result) {
                return true;
            }
            result.pop();
        }
        false
    }

    fn is_visited(
        start: &String,
        index: usize,
        route_map: &BTreeMap<String, Vec<String>>,
        result: &Vec<String>,
    ) -> bool {
        let destinations = match route_map.get(start) {
            Some(d) => d,
            None => return false,
        };
        let target = &destinations[index];
        let mut used = 0;
        for i in 0..index {
            if &destinations[i] == target {
                used += 1;
            }
        }
        let mut count_in_result = 0;
        for i in 0..result.len() - 1 {
            if &result[i] == start && &result[i + 1] == target {
                count_in_result += 1;
            }
        }
        count_in_result > used
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_itinerary() {
        let tickets = vec![
            vec!["JFK".to_string(), "SFO".to_string()],
            vec!["JFK".to_string(), "ATL".to_string()],
            vec!["SFO".to_string(), "ATL".to_string()],
            vec!["ATL".to_string(), "JFK".to_string()],
            vec!["ATL".to_string(), "SFO".to_string()],
        ];
        let expected = vec!["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"];
        assert_eq!(Solution::find_itinerary(tickets), expected);
    }
}
