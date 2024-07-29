use crate::solutions::Solution;

impl Solution {
    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        fn to_equation(words: &Vec<String>, result: &str) -> Vec<(usize, usize)> {
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
                .map(|(_, (min, max))| (*min, *max))
                .filter(|(min, max)| min != max)
                .collect()
        }

        fn is_valid_answer(equation: &[(usize, usize)], answer: &[usize]) -> bool {
            let mut left_hand = 0;
            let mut right_hand = 0;
            equation.iter().zip(answer).for_each(|((min, max), value)| {
                left_hand += min * value;
                right_hand += max * value;
            });
            left_hand == right_hand
        }

        fn is_solvable_backtrack(
            equation: &[(usize, usize)],
            answer: &mut Vec<usize>,
            used: &mut std::collections::HashSet<usize>,
        ) -> bool {
            if answer.len() == equation.len() {
                return is_valid_answer(equation, answer);
            }
            for i in 0..10 {
                if used.contains(&i) {
                    continue;
                }
                answer.push(i);
                used.insert(i);
                if is_solvable_backtrack(equation, answer, used) {
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
        let res = is_solvable_backtrack(&equation, &mut answer, &mut used);
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
