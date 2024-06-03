use crate::solutions::Solution;

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let t = t.chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut j = 0;
        while i < s.len() && j < t.len() {
            if s[i] == t[j] {
                i += 1;
                j += 1;
            } else {
                i += 1;
            }
        }
        (t.len() - j) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_characters() {
        let s = "abc".to_string();
        let t = "abc".to_string();
        let res = 0;
        assert_eq!(Solution::append_characters(s, t), res);
    }

    #[test]
    fn test_append_characters_2() {
        let s = "coaching".to_string();
        let t = "coding".to_string();
        let res = 4;
        assert_eq!(Solution::append_characters(s, t), res);
    }

    #[test]
    fn test_append_characters_3() {
        let s = "abcde".to_string();
        let t = "a".to_string();
        let res = 0;
        assert_eq!(Solution::append_characters(s, t), res);
    }

    #[test]
    fn test_append_characters_4() {
        let s = "z".to_string();
        let t = "abcde".to_string();
        let res = 5;
        assert_eq!(Solution::append_characters(s, t), res);
    }
}
