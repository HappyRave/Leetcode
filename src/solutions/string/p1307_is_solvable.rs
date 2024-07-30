use crate::solutions::Solution;

impl Solution {
    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        fn to_equation(words: &Vec<String>, result: &str) -> Vec<(char, usize, usize)> {
            let mut char_map: std::collections::HashMap<char, (usize, usize)> =
                std::collections::HashMap::with_capacity(10);
            for word in words {
                for c in word.chars() {
                    char_map.entry(c).or_insert((0, 0)).0 += 1;
                }
            }
            for c in result.chars() {
                char_map.entry(c).or_insert((0, 0)).1 += 1;
            }
            println!("{:?}", char_map);
            char_map
                .iter()
                .map(|(c, (min, max))| (*c, *min, *max))
                //.filter(|(min, max)| min != max)
                .collect()
        }

        fn is_valid_answer(
            equation: &[(char, usize, usize)],
            answer: &[usize],
            words: &Vec<String>,
            result: &str,
        ) -> bool {
            fn word_to_num(
                word: &str,
                equation: &[(char, usize, usize)],
                answer: &[usize],
            ) -> usize {
                let mut word = word.clone().to_string();
                for i in 0..equation.len() {
                    let (c, _, _) = equation[i];
                    let num = answer[i];
                    //println!("{:?} {:?}", c, num);
                    word = word.replace(c, &num.to_string());
                }
                //println!("{:?}", word);
                word.parse().unwrap()
            }
            let left_hand_side = words
                .iter()
                .map(|word| word_to_num(word, equation, answer))
                .sum::<usize>();
            let right_hand_side = word_to_num(result, equation, answer);
            left_hand_side == right_hand_side
        }

        fn is_solvable_backtrack(
            equation: &[(char, usize, usize)],
            answer: &mut Vec<usize>,
            used: &mut std::collections::HashSet<usize>,
            words: &Vec<String>,
            result: &str,
        ) -> bool {
            if answer.len() == equation.len() {
                return is_valid_answer(equation, answer, words, result);
            }
            for i in 0..10 {
                if used.contains(&i) {
                    continue;
                }
                answer.push(i);
                used.insert(i);
                if is_solvable_backtrack(equation, answer, used, words, result) {
                    return true;
                }
                answer.pop();
                used.remove(&i);
            }
            false
        }

        let equation = to_equation(&words, &result);
        println!("{:?}", equation);
        let mut used: std::collections::HashSet<usize> = std::collections::HashSet::new();
        let mut answer = vec![];
        let res = is_solvable_backtrack(&equation, &mut answer, &mut used, &words, &result);
        println!("{:?}", answer);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_solvable() {
        let words = vec!["SEND".to_string(), "MORE".to_string()];
        let result = "MONEY".to_string();
        assert!(Solution::is_solvable(words, result));
    }

    #[test]
    fn test_is_solvable_2() {
        let words = vec!["SIX".to_string(), "SEVEN".to_string(), "SEVEN".to_string()];
        let result = "TWENTY".to_string();
        assert!(Solution::is_solvable(words, result));
    }

    #[test]
    fn test_is_solvable_3() {
        let words = vec!["LEET".to_string(), "CODE".to_string()];
        let result = "POINT".to_string();
        assert!(!Solution::is_solvable(words, result));
    }
}
