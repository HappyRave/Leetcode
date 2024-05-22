use crate::solutions::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }
        let mut prefix = strs[0].clone();
        (1..strs.len()).for_each(|i| {
            let mut j = 0;
            let mut new_prefix = String::new();
            for c in strs[i].chars() {
                if let Some(c2) = prefix.chars().nth(j) {
                    if c == c2 {
                        new_prefix.push(c);
                        j += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            prefix = new_prefix;
        });
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let strs = ["flower", "flow", "flight"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(Solution::longest_common_prefix(strs), "fl".to_string());
    }

    #[test]
    fn test_2() {
        let strs = ["dog", "racecar", "car"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(Solution::longest_common_prefix(strs), "".to_string());
    }
}
