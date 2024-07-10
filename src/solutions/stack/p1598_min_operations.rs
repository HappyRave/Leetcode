use crate::solutions::Solution;

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        logs.iter()
            .fold(0_usize, |acc: usize, log: &String| match log.as_str() {
                "./" => acc,
                "../" => acc.saturating_sub(1),
                _ => acc + 1,
            }) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_operations() {
        let logs = vec![
            "d1/".to_string(),
            "d2/".to_string(),
            "../".to_string(),
            "d21/".to_string(),
            "./".to_string(),
        ];
        assert_eq!(Solution::min_operations(logs), 2);
    }

    #[test]
    fn test_min_operations_2() {
        let logs = vec![
            "d1/".to_string(),
            "d2/".to_string(),
            "./".to_string(),
            "d3/".to_string(),
            "../".to_string(),
            "d31/".to_string(),
        ];
        assert_eq!(Solution::min_operations(logs), 3);
    }

    #[test]
    fn test_min_operations_3() {
        let logs = vec![
            "d1/".to_string(),
            "../".to_string(),
            "../".to_string(),
            "../".to_string(),
        ];
        assert_eq!(Solution::min_operations(logs), 0);
    }
}
