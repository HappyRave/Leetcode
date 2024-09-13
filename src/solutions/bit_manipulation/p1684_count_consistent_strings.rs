use crate::solutions::Solution;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut count = 0;
        let mut mask = 0;
        for c in allowed.chars() {
            mask |= 1 << (c as u8 - b'a');
        }
        for word in words {
            let mut is_consistent = true;
            for c in word.chars() {
                if mask & (1 << (c as u8 - b'a')) == 0 {
                    is_consistent = false;
                    break;
                }
            }
            if is_consistent {
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
    fn test_count_consistent_strings() {
        let allowed = "ab".to_string();
        let words = vec![
            "ad".to_string(),
            "bd".to_string(),
            "aaab".to_string(), // cspell: disable-line
            "baa".to_string(),
            "badab".to_string(), // cspell: disable-line
        ];
        let res = 2;
        assert_eq!(Solution::count_consistent_strings(allowed, words), res);
    }

    #[test]
    fn test_count_consistent_strings_2() {
        let allowed = "cad".to_string();
        let words = vec![
            "cc".to_string(),
            "acd".to_string(),
            "b".to_string(),
            "ba".to_string(),
            "bac".to_string(),
            "bad".to_string(),
            "ac".to_string(),
            "d".to_string(),
        ];
        let res = 4;
        assert_eq!(Solution::count_consistent_strings(allowed, words), res);
    }
}
