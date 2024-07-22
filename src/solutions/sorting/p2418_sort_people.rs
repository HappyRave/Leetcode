use crate::solutions::Solution;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut people = names.into_iter().zip(heights).collect::<Vec<_>>();
        people.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        people.into_iter().map(|(name, _)| name).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_people() {
        let names = vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()];
        let heights = vec![180, 165, 170];
        let result = Solution::sort_people(names, heights);
        let expected = vec!["Mary".to_string(), "Emma".to_string(), "John".to_string()];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sort_people_2() {
        let names = vec!["Alice".to_string(), "Bob".to_string(), "Bob".to_string()];
        let heights = vec![155,185,150];
        let result = Solution::sort_people(names, heights);
        let expected = vec!["Bob".to_string(), "Alice".to_string(), "Bob".to_string()];
        assert_eq!(result, expected);
    }
}
