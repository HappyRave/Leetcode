use crate::solutions::Solution;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count_a = s.iter().filter(|&&c| c == b'a').count();
        let mut count_b = 0;
        let mut result = count_a;

        for &c in s {
            if c == b'a' {
                count_a -= 1;
            } else {
                count_b += 1;
            }
            result = result.min(count_a + count_b);
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_deletions() {
        let s = "aab".to_string();
        assert_eq!(Solution::minimum_deletions(s), 0);
    }

    #[test]
    fn test_minimum_deletions_2() {
        let s = "aababbab".to_string(); // cspell: disable-line
        assert_eq!(Solution::minimum_deletions(s), 2);
    }

    #[test]
    fn test_minimum_deletions_3() {
        let s = "bbaaaaabb".to_string(); // cspell: disable-line
        assert_eq!(Solution::minimum_deletions(s), 2);
    }
}
