pub struct Solution;

impl Solution {
    fn find_generate_parenthesis(
        lindex: i32,
        rindex: i32,
        str: &mut String,
        res: &mut Vec<String>,
    ) {
        if lindex == 0 && rindex == 0 {
            res.push(str.clone());
            return;
        }
        if lindex > 0 {
            str.push('(');
            Self::find_generate_parenthesis(lindex - 1, rindex, str, res);
            str.pop();
        }
        if rindex > 0 && lindex < rindex {
            str.push(')');
            Self::find_generate_parenthesis(lindex, rindex - 1, str, res);
            str.pop();
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 0 {
            return vec![];
        }
        let mut res = vec![];
        let mut s = String::new();
        Self::find_generate_parenthesis(n, n, &mut s, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_generate_parenthesis() {
        let mut actual = Solution::generate_parenthesis(3);
        actual.sort();
        let mut expected = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
        expected.sort();
        assert_eq!(actual, expected);
    }
}
