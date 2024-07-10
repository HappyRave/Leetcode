use crate::solutions::Solution;

impl Solution {
    pub fn check_string(s: String) -> bool {
        let mut a = true;
        for c in s.chars() {
            if c == 'b' {
                a = false;
            } else if !a {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_string() {
        let s = "aabb".to_string();
        assert!(Solution::check_string(s));
    }

    #[test]
    fn test_check_string_2() {
        let s = "bb".to_string();
        assert!(Solution::check_string(s));
    }

    #[test]
    fn test_check_string_3() {
        let s = "ba".to_string();
        assert!(!Solution::check_string(s));
    }
}
