use crate::solutions::Solution;

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut trie = Trie::new();
        for word in &dictionary {
            trie.insert(word);
        }

        let mut result = String::new();
        for word in sentence.split_whitespace() {
            if !result.is_empty() {
                result.push(' ');
            }
            result.push_str(&trie.search(word));
        }

        result
    }
}

pub struct Trie {
    children: [Option<Box<Trie>>; 26],
    prefix: String,
    is_end: bool,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            children: Default::default(),
            prefix: String::new(),
            is_end: false,
        }
    }

    pub fn from_prefix(prefix: String) -> Self {
        Self {
            children: Default::default(),
            prefix,
            is_end: false,
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        let mut prefix = String::new();
        for ch in word.chars() {
            prefix.push(ch);
            let i = (ch as u8 - b'a') as usize;
            if node.children[i].is_none() {
                node.children[i] = Some(Box::new(Trie::from_prefix(prefix.clone())));
            }
            node = node.children[i].as_deref_mut().unwrap();
        }
        node.is_end = true;
    }

    pub fn search(&self, word: &str) -> String {
        let mut node = self;
        for ch in word.chars() {
            let i = (ch as u8 - b'a') as usize;
            match node.children[i].as_deref() {
                Some(next) => {
                    if next.is_end {
                        return next.prefix.clone();
                    }
                    node = next;
                }
                None => {
                    if node.is_end {
                        return node.prefix.clone();
                    } else {
                        return word.to_string();
                    }
                }
            }
        }
        node.prefix.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_words() {
        assert_eq!(
            Solution::replace_words(
                vec!["cat".to_string(), "bat".to_string(), "rat".to_string()],
                "the cattle was rattled by the battery".to_string()
            ),
            "the cat was rat by the bat".to_string()
        );
    }

    #[test]
    fn test_replace_words_2() {
        assert_eq!(
            Solution::replace_words(
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
                "aadsfasf absbs bbab cadsfafs".to_string()
            ),
            "a a b c".to_string()
        );
    }

    #[test]
    fn test_replace_words_3() {
        assert_eq!(
            Solution::replace_words(
                vec![
                    "a".to_string(),
                    "aa".to_string(),
                    "aaa".to_string(),
                    "aaaa".to_string()
                ],
                "a aa a aaaa aaa aaa aaa aaaaaa bbb baba ababa".to_string()
            ),
            "a a a a a a a a bbb baba a".to_string()
        );
    }
}
