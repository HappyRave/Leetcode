use crate::solutions::Solution;

impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        fn letter_idx(letter: char) -> usize {
            letter as usize - 'a' as usize
        }

        let letter_count = {
            let mut letter_count = vec![0i8; 26];
            for letter in letters {
                letter_count[letter_idx(letter)] += 1;
            }
            letter_count
        };

        fn backtrack(
            words: &Vec<String>,
            score: &Vec<i32>,
            letter_count: &Vec<i8>,
            word_idx: usize,
            current_score: i32,
        ) -> i32 {
            if word_idx == words.len() {
                return current_score;
            }

            let mut max_score = 0;
            let mut letter_count_copy = letter_count.clone();
            let word = &words[word_idx];
            let mut word_score = 0;
            let mut can_form_word = true;

            for letter in word.chars() {
                let idx = letter_idx(letter);
                if letter_count_copy[idx] == 0 {
                    can_form_word = false;
                    break;
                }

                word_score += score[idx];
                letter_count_copy[idx] -= 1;
            }

            if can_form_word {
                max_score = backtrack(
                    words,
                    score,
                    &letter_count_copy,
                    word_idx + 1,
                    current_score + word_score,
                );
            }

            max_score = std::cmp::max(
                max_score,
                backtrack(words, score, letter_count, word_idx + 1, current_score),
            );

            max_score
        }

        backtrack(&words, &score, &letter_count, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_score_words() {
        assert_eq!(
            Solution::max_score_words(
                vec![
                    "dog".to_string(),
                    "cat".to_string(),
                    "dad".to_string(),
                    "good".to_string()
                ],
                vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'],
                vec![1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ),
            23
        );
    }

    #[test]
    fn test_max_score_words_2() {
        assert_eq!(
            Solution::max_score_words(
                vec![
                    "xxxz".to_string(),
                    "ax".to_string(),
                    "bx".to_string(),
                    "cx".to_string()
                ],
                vec!['z', 'a', 'b', 'c', 'x', 'x', 'x'],
                vec![4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10]
            ),
            27
        );
    }

    #[test]
    fn test_max_score_words_3() {
        assert_eq!(
            Solution::max_score_words(
                vec!["leetcode".to_string()],
                vec!['l', 'e', 't', 'c', 'o', 'd'],
                vec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]
            ),
            0
        );
    }
}
