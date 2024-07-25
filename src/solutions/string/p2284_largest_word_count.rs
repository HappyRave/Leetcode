use crate::solutions::Solution;

impl Solution {
    pub fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {
        let map: std::collections::HashMap<String, usize> =
            std::collections::HashMap::with_capacity(
                senders.len().checked_div(2).unwrap_or_default(),
            );
        messages
            .iter()
            .map(|message| message.split_whitespace().count())
            .zip(senders)
            .fold(
                map,
                |mut acc: std::collections::HashMap<String, usize>, (count, sender)| {
                    *acc.entry(sender).or_default() += count;
                    acc
                },
            )
            .into_iter()
            .max_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)))
            .unwrap_or_default()
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_word_count() {
        let messages = vec!["Hello how are you".to_string(), "I am fine".to_string()];
        let senders = vec!["John".to_string(), "Alice".to_string()];
        let result = "John".to_string();
        assert_eq!(Solution::largest_word_count(messages, senders), result);
    }

    #[test]
    fn test_largest_word_count_2() {
        let messages = vec![
            "Hello userTwooo".to_string(), // cspell: disable-line
            "Hi userThree".to_string(),
            "Wonderful day Alice".to_string(),
            "Nice day userThree".to_string(),
        ];
        let senders = vec![
            "Alice".to_string(),
            "userTwo".to_string(),
            "userThree".to_string(),
            "Alice".to_string(),
        ];
        let result = "Alice".to_string();
        assert_eq!(Solution::largest_word_count(messages, senders), result);
    }

    #[test]
    fn test_largest_word_count_3() {
        let messages = vec![
            "How is leetcode for everyone".to_string(),
            "Leetcode is useful for practice".to_string(),
        ];
        let senders = vec!["Charlie".to_string(), "Bob".to_string()];
        let result = "Charlie".to_string();
        assert_eq!(Solution::largest_word_count(messages, senders), result);
    }

    #[test]
    fn test_largest_word_count_4() {
        let messages = vec![];
        let senders = vec![];
        let result = "".to_string();
        assert_eq!(Solution::largest_word_count(messages, senders), result);
    }
}
