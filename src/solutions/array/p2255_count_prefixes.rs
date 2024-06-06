use crate::solutions::Solution;

impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        let mut count = 0;
        for word in &words {
            if s.starts_with(word) {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_prefixes() {
        let words = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "ab".to_string(),
            "bc".to_string(),
            "abc".to_string(),
        ];
        let s = "abc".to_string();
        assert_eq!(Solution::count_prefixes(words, s), 3);
    }

    #[test]
    fn test_count_prefixes_2() {
        let words = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "ab".to_string(),
            "bc".to_string(),
            "abc".to_string(),
        ];
        let s = "def".to_string();
        assert_eq!(Solution::count_prefixes(words, s), 0);
    }

    #[test]
    fn test_count_prefixes_3() {
        let words = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "ab".to_string(),
            "bc".to_string(),
            "abc".to_string(),
        ];
        let s = "a".to_string();
        assert_eq!(Solution::count_prefixes(words, s), 1);
    }

    #[test]
    fn test_count_prefixes_4() {
        let words = vec!["a".to_string(), "a".to_string()];
        let s = "aa".to_string();
        assert_eq!(Solution::count_prefixes(words, s), 2);
    }
}
