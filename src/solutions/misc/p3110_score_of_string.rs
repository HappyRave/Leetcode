use crate::solutions::Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        s.as_bytes()
            .windows(2)
            .map(|a| a[0].abs_diff(a[1]) as i32)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_of_string() {
        let s = "hello".to_string();
        assert_eq!(Solution::score_of_string(s), 13);
    }

    #[test]
    fn test_score_of_string_2() {
        let s = "zaz".to_string();
        assert_eq!(Solution::score_of_string(s), 50);
    }
}
