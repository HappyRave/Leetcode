use crate::solutions::Solution;

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let mut i = 0;
        let n = formula.len();
        let mut stack = vec![std::collections::HashMap::new()];
        let formula = formula.chars().collect::<Vec<char>>();
        while i < n {
            match formula[i] {
                '(' => {
                    stack.push(std::collections::HashMap::new());
                    i += 1;
                }
                ')' => {
                    i += 1;
                    let mut num = 0;
                    while i < n && formula[i].is_ascii_digit() {
                        num = num * 10 + formula[i].to_digit(10).unwrap();
                        i += 1;
                    }
                    let top = stack.pop().unwrap();
                    let map = stack.last_mut().unwrap();
                    for (k, v) in top {
                        *map.entry(k).or_insert(0) += v * num.max(1);
                    }
                }
                _ => {
                    let start = i;
                    i += 1;
                    while i < n && formula[i].is_ascii_lowercase() {
                        i += 1;
                    }
                    let element = formula[start..i].iter().collect::<String>();
                    let mut num = 0;
                    while i < n && formula[i].is_ascii_digit() {
                        num = num * 10 + formula[i].to_digit(10).unwrap();
                        i += 1;
                    }
                    *stack.last_mut().unwrap().entry(element).or_insert(0) += num.max(1);
                }
            }
        }
        let mut result: Vec<_> = stack.pop().unwrap().into_iter().collect();
        result.sort_unstable();
        result
            .into_iter()
            .map(|(s, c)| {
                s + &(if c != 1 {
                    c.to_string()
                } else {
                    "".to_string()
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_of_atoms() {
        let formula = "H2O".to_string();
        let result = Solution::count_of_atoms(formula);
        assert_eq!(result, "H2O".to_string());
    }

    #[test]
    fn test_count_of_atoms_2() {
        let formula = "Mg(OH)2".to_string();
        let result = Solution::count_of_atoms(formula);
        assert_eq!(result, "H2MgO2".to_string());
    }

    #[test]
    fn test_count_of_atoms_3() {
        let formula = "K4(ON(SO3)2)2".to_string();
        let result = Solution::count_of_atoms(formula);
        assert_eq!(result, "K4N2O14S4".to_string());
    }
}
