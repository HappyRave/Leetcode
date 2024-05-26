use crate::solutions::Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        fn process(s: &str) -> String {
            let mut stack = Vec::new();
            for c in s.chars() {
                if c == '#' {
                    stack.pop();
                } else {
                    stack.push(c);
                }
            }
            stack.into_iter().collect()
        }
        process(&s) == process(&t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backspace_compare() {
        assert!(Solution::backspace_compare(
            "ab#c".to_string(),
            "ad#c".to_string()
        ));
    }

    #[test]
    fn test_backspace_compare_2() {
        assert!(Solution::backspace_compare(
            "ab##".to_string(),
            "c#d#".to_string()
        ));
    }

    #[test]
    fn test_backspace_compare_3() {
        assert!(Solution::backspace_compare(
            "a##c".to_string(),
            "#a#c".to_string()
        ));
    }

    #[test]
    fn test_backspace_compare_4() {
        assert!(!Solution::backspace_compare(
            "a#c".to_string(),
            "b".to_string()
        ));
    }
}
