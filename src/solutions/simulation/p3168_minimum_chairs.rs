use crate::solutions::Solution;

impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        s.bytes()
            .fold((0, 0), |(max_chairs, current_chairs), c| match c {
                b'E' => (max_chairs.max(current_chairs + 1), current_chairs + 1),
                _ => (max_chairs, current_chairs - 1),
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_chairs() {
        assert_eq!(Solution::minimum_chairs("EEEEEEE".to_string()), 7);
    }

    #[test]
    fn test_minimum_chairs_2() {
        assert_eq!(Solution::minimum_chairs("ELELEEL".to_string()), 2); // cspell: disable-line
    }

    #[test]
    fn test_minimum_chairs_3() {
        assert_eq!(Solution::minimum_chairs("ELEELEELLL".to_string()), 3); // cspell: disable-line
    }
}
