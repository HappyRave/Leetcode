use crate::solutions::Solution;

impl Solution {
    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        fn to_equation(words: &[String], result: &str) -> Vec<(i32, bool)> {
            let mut char_map: std::collections::HashMap<char, (i32, bool)> =
                std::collections::HashMap::with_capacity(10);
            let mut multiplier: i32;

            for word in words {
                multiplier = 1;
                for c in word.chars().rev() {
                    let entry = char_map.entry(c).or_default();
                    entry.0 += multiplier;
                    multiplier *= 10;
                }
                if let Some(first_char) = word.chars().next() {
                    char_map.get_mut(&first_char).unwrap().1 = word.len() > 1;
                }
            }

            multiplier = 1;
            for c in result.chars().rev() {
                let entry = char_map.entry(c).or_default();
                entry.0 -= multiplier;
                multiplier *= 10;
            }
            if let Some(first_char) = result.chars().next() {
                char_map.get_mut(&first_char).unwrap().1 = result.len() > 1;
            }

            char_map
                .into_iter()
                .filter(|(_, v)| v.0 != 0)
                .map(|(_, v)| v)
                .collect()
        }

        fn is_solvable_backtrack(
            equation: &mut [(i32, bool)],
            answer: &mut Vec<usize>,
            used: &mut std::collections::HashSet<usize>,
            sum: i32,
        ) -> bool {
            if answer.len() == equation.len() {
                return sum == 0;
            }

            for i in 0..10 {
                if used.contains(&i) {
                    continue;
                }
                if equation[answer.len()].1 && i == 0 {
                    continue;
                }
                answer.push(i);
                used.insert(i);
                let sum = sum + equation[answer.len() - 1].0 * i as i32;
                if sum.abs() > equation[answer.len() - 1].0.abs() * 11 {
                    answer.pop();
                    used.remove(&i);
                    return false;
                }
                if is_solvable_backtrack(equation, answer, used, sum) {
                    return true;
                }
                answer.pop();
                used.remove(&i);
            }
            false
        }

        //let timer = std::time::Instant::now();
        let mut equation = to_equation(&words, &result);
        equation.sort_by_key(|v| -v.0.abs());
        //println!("Time to create equation: {:?}", timer.elapsed());
        //let timer = std::time::Instant::now();
        let mut used: std::collections::HashSet<usize> = std::collections::HashSet::new();
        let mut answer = vec![];
        // let ret = is_solvable_backtrack(&mut equation, &mut answer, &mut used, 0);
        // println!("Time to solve: {:?}", timer.elapsed());
        // ret
        is_solvable_backtrack(&mut equation, &mut answer, &mut used, 0)
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

    #[test]
    fn test_is_solvable_4() {
        let words = vec!["CBA".to_string(); 5];
        let result = "EDD".to_string();
        assert!(!Solution::is_solvable(words, result));
    }

    #[test]
    fn test_is_solvable_5() {
        let words = vec![
            "SEIS".to_string(),    // cspell: disable-line
            "CATORCE".to_string(), // cspell: disable-line
            "SETENTA".to_string(), // cspell: disable-line
        ];
        let result = "NOVENTA".to_string(); // cspell: disable-line
        assert!(Solution::is_solvable(words, result));
    }

    #[test]
    fn test_is_solvable_6() {
        let words = vec!["AB".to_string(), "CD".to_string(), "EF".to_string()];
        let result = "GHIJ".to_string(); // cspell: disable-line
        assert!(!Solution::is_solvable(words, result));
    }

    #[test]
    fn test_is_solvable_7() {
        let words = vec!["GEMINI".to_string(), "VIRGO".to_string()];
        let result = "CANCER".to_string();
        assert!(Solution::is_solvable(words, result));
    }

    #[test]
    fn test_is_solvable_8() {
        let words: Vec<String> = vec!["A".to_string(), "B".to_string()];
        let result = "A".to_string();
        assert!(Solution::is_solvable(words, result));
    }
}
