use crate::solutions::Solution;

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        s.as_bytes()
            .chunks(2)
            .filter(|chunk| chunk[0] != chunk[1])
            .count() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_changes() {
        let s = "1010".to_string();
        assert_eq!(Solution::min_changes(s), 2);
    }

    #[test]
    fn test_min_changes_2() {
        let s = "1111".to_string();
        assert_eq!(Solution::min_changes(s), 0);
    }

    #[test]
    fn test_min_changes_3() {
        let s = "1001".to_string();
        assert_eq!(Solution::min_changes(s), 2);
    }

    #[test]
    fn test_min_changes_4() {
        let s = "10".to_string();
        assert_eq!(Solution::min_changes(s), 1);
    }
}
