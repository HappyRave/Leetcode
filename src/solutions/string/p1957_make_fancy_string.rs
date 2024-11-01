use crate::solutions::Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut result = String::new();
        let mut count = 0;
        for c in s.chars() {
            if let Some(last) = result.chars().last() {
                if c == last && count >= 2 {
                    continue;
                }
                if c != last {
                    count = 0;
                }
            }

            result.push(c);
            count += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_fancy_string() {
        let s = "leeetcode".to_string(); // cspell: disable-line
        assert_eq!(Solution::make_fancy_string(s), "leetcode".to_string());
    }

    #[test]
    fn test_make_fancy_string_2() {
        let s = "aaabaaaa".to_string(); // cspell: disable-line
        assert_eq!(Solution::make_fancy_string(s), "aabaa".to_string()); // cspell: disable-line
    }

    #[test]
    fn test_make_fancy_string_3() {
        let s = "aab".to_string();
        assert_eq!(Solution::make_fancy_string(s), "aab".to_string());
    }
}
