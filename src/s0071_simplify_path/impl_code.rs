pub struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let parts: Vec<&str> = path.split('/').collect();
        let mut stack: Vec<&str> = Vec::new();

        for part in parts {
            if part == ".." {
                if !stack.is_empty() {
                    stack.pop();
                }
            } else if part != "." && !part.is_empty() {
                stack.push(part);
            }
        }

        if stack.is_empty() {
            return "/".to_string();
        }

        "/".to_string() + &stack.join("/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplify_path() {
        assert_eq!(Solution::simplify_path("/home/".to_string()), "/home");
        assert_eq!(Solution::simplify_path("/../".to_string()), "/");
        assert_eq!(
            Solution::simplify_path("/home//foo/".to_string()),
            "/home/foo"
        );
    }
}
