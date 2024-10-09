use crate::solutions::Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut stack = Vec::new();
        for c in s.bytes() {
            match (stack.last(), c) {
                (Some(&b'('), b')') => {
                    stack.pop();
                }
                _ => {
                    stack.push(c);
                }
            }
        }
        stack.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_add_to_make_valid() {
        let s = "())".to_string();
        let result = 1;
        assert_eq!(Solution::min_add_to_make_valid(s), result);
    }

    #[test]
    fn test_min_add_to_make_valid_2() {
        let s = "(((".to_string();
        let result = 3;
        assert_eq!(Solution::min_add_to_make_valid(s), result);
    }

    #[test]
    fn test_min_add_to_make_valid_3() {
        let s = "()".to_string();
        let result = 0;
        assert_eq!(Solution::min_add_to_make_valid(s), result);
    }

    #[test]
    fn test_min_add_to_make_valid_4() {
        let s = "()))((".to_string();
        let result = 4;
        assert_eq!(Solution::min_add_to_make_valid(s), result);
    }
}
