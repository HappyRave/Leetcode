use crate::solutions::Solution;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let mut min_freq = [usize::MAX; 26];

        for word in &words {
            let mut freq = [0; 26];
            for c in word.chars() {
                freq[c as usize - 'a' as usize] += 1;
            }

            for i in 0..26 {
                min_freq[i] = min_freq[i].min(freq[i]);
            }
        }

        (0..26).for_each(|i| {
            for _ in 0..min_freq[i] {
                result.push(((i as u8 + b'a') as char).to_string());
            }
        });

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_chars() {
        assert_eq!(
            Solution::common_chars(vec![
                "bella".to_string(),
                "label".to_string(),
                "roller".to_string()
            ]),
            vec!["e".to_string(), "l".to_string(), "l".to_string()]
        );
    }

    #[test]
    fn test_common_chars_2() {
        assert_eq!(
            Solution::common_chars(vec![
                "cool".to_string(),
                "lock".to_string(),
                "cook".to_string()
            ]),
            vec!["c".to_string(), "o".to_string()]
        );
    }
}
