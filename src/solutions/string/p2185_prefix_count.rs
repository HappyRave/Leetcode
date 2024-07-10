use crate::solutions::Solution;

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.iter().filter(|word| word.starts_with(&pref)).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_count() {
        let words = vec![
            "apple".to_string(),
            "app".to_string(),
            "apricot".to_string(),
            "apocalypse".to_string(),
        ];
        let pref = "ap".to_string();
        assert_eq!(Solution::prefix_count(words, pref), 4);
    }

    #[test]
    fn test_prefix_count_2() {
        let words = vec![
            "apple".to_string(),
            "app".to_string(),
            "apricot".to_string(),
            "apocalypse".to_string(),
        ];
        let pref = "app".to_string();
        assert_eq!(Solution::prefix_count(words, pref), 2);
    }

    #[test]
    fn test_prefix_count_3() {
        let words = vec![
            "apple".to_string(),
            "app".to_string(),
            "apricot".to_string(),
            "apocalypse".to_string(),
        ];
        let pref = "apocalypse".to_string();
        assert_eq!(Solution::prefix_count(words, pref), 1);
    }

    #[test]
    fn test_prefix_count_4() {
        let words = vec![
            "leetcode".to_string(),
            "leet".to_string(),
            "le".to_string(),
            "leet".to_string(),
        ];
        let pref = "code".to_string();
        assert_eq!(Solution::prefix_count(words, pref), 0);
    }
}
