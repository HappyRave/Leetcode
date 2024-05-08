use super::Solution;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut score = score.into_iter().enumerate().collect::<Vec<_>>();
        score.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        let mut result = vec![String::new(); score.len()];
        for (i, (index, _)) in score.iter().enumerate() {
            result[*index] = match i {
                0 => "Gold Medal".to_string(),
                1 => "Silver Medal".to_string(),
                2 => "Bronze Medal".to_string(),
                _ => (i + 1).to_string(),
            };
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_relative_ranks() {
        let score = vec![5, 4, 3, 2, 1];
        let result = Solution::find_relative_ranks(score);
        let expected = vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_relative_ranks_unordered() {
        let score = vec![1, 5, 4, 3, 2];
        let result = Solution::find_relative_ranks(score);
        let expected = vec!["5", "Gold Medal", "Silver Medal", "Bronze Medal", "4"];
        assert_eq!(result, expected);
    }
}
