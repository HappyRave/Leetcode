use crate::solutions::Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        fn backtrack(
            s: &str,
            word_dict: &Vec<String>,
            start: usize,
            path: &mut Vec<String>,
            res: &mut Vec<String>,
        ) {
            if start == s.len() {
                res.push(path.join(" "));
                return;
            }
            for word in word_dict.iter() {
                if s[start..].starts_with(word) {
                    path.push(word.to_string());
                    backtrack(s, word_dict, start + word.len(), path, res);
                    path.pop();
                }
            }
        }

        let mut res = vec![];
        let mut path = vec![];
        backtrack(&s, &word_dict, 0, &mut path, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_break() {
        let s = "catsanddog".to_string();
        let word_dict = ["cat", "cats", "and", "sand", "dog"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        let mut res = vec!["cats and dog", "cat sand dog"];
        let mut actual = Solution::word_break(s, word_dict);
        res.sort();
        actual.sort();
        assert_eq!(actual, res);
    }

    #[test]
    fn test_word_break_2() {
        let s = "pineapplepenapple".to_string();
        let word_dict = ["apple", "pen", "applepen", "pine", "pineapple"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        let mut res = vec![
            "pine apple pen apple",
            "pineapple pen apple",
            "pine applepen apple",
        ];
        let mut actual = Solution::word_break(s, word_dict);
        res.sort();
        actual.sort();
        assert_eq!(actual, res);
    }

    #[test]
    fn test_word_break_3() {
        let s = "catsandog".to_string();
        let word_dict = ["cats", "dog", "sand", "and", "cat"]
            .iter()
            .map(|&s| s.to_string())
            .collect();
        let res: Vec<&str> = vec![];
        assert_eq!(Solution::word_break(s, word_dict), res);
    }
}
