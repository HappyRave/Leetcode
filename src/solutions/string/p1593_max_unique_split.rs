use crate::solutions::Solution;

impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        fn dfs(s: &str, set: &mut std::collections::HashSet<String>, max: &mut i32) {
            if s.is_empty() {
                *max = std::cmp::max(*max, set.len() as i32);
                return;
            }
            for i in 1..=s.len() {
                let (left, right) = s.split_at(i);
                if set.insert(left.to_string()) {
                    dfs(right, set, max);
                    set.remove(left);
                }
            }
        }

        let mut set = std::collections::HashSet::new();
        let mut max = 0;
        dfs(&s, &mut set, &mut max);
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_unique_split() {
        assert_eq!(Solution::max_unique_split("ababccc".to_string()), 5); // cspell: disable-line
    }

    #[test]
    fn test_max_unique_split_2() {
        assert_eq!(Solution::max_unique_split("aba".to_string()), 2);
    }

    #[test]
    fn test_max_unique_split_3() {
        assert_eq!(Solution::max_unique_split("aa".to_string()), 1);
    }
}
