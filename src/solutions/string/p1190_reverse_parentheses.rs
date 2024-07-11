use crate::solutions::Solution;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stack = vec![];
        let mut res = String::new();
        for c in s.chars() {
            if c == ')' {
                let mut tmp = vec![];
                while let Some(top) = stack.pop() {
                    if top == '(' {
                        break;
                    }
                    tmp.push(top);
                }
                stack.append(&mut tmp);
            } else {
                stack.push(c);
            }
        }
        for c in stack {
            res.push(c);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_parentheses() {
        assert_eq!(
            Solution::reverse_parentheses("(abcd)".to_string()),
            "dcba".to_string() // cspell: disable-line
        );
    }

    #[test]
    fn test_reverse_parentheses_2() {
        assert_eq!(
            Solution::reverse_parentheses("(u(love)i)".to_string()),
            "iloveu".to_string() // cspell: disable-line
        );
    }

    #[test]
    fn test_reverse_parentheses_3() {
        assert_eq!(
            Solution::reverse_parentheses("(ed(et(oc))el)".to_string()),
            "leetcode".to_string()
        );
    }
}
