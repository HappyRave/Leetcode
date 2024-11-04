use crate::solutions::Solution;

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut result = String::new();
        let mut count = 1;
        let mut chars = word.chars();
        let mut prev = chars.next().unwrap();
        for c in chars {
            if c == prev && count < 9 {
                count += 1;
            } else {
                result.push_str(&count.to_string());
                result.push(prev);
                prev = c;
                count = 1;
            }
        }
        result.push_str(&count.to_string());
        result.push(prev);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compressed_string() {
        let word = "aaabcccd".to_string(); // cspell: disable-line
        assert_eq!(Solution::compressed_string(word), "3a1b3c1d".to_string());
    }

    #[test]
    fn test_compressed_string_2() {
        let word = "aaabcccdde".to_string(); // cspell: disable-line
        assert_eq!(Solution::compressed_string(word), "3a1b3c2d1e".to_string());
    }

    #[test]
    fn test_compressed_string_3() {
        let word = "abcde".to_string();
        assert_eq!(Solution::compressed_string(word), "1a1b1c1d1e".to_string());
    }

    #[test]
    fn test_compressed_string_4() {
        let word = "aaaaaaaaaaaaaabb".to_string(); // cspell: disable-line
        assert_eq!(Solution::compressed_string(word), "9a5a2b".to_string());
    }
}
