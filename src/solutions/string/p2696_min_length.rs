use crate::solutions::Solution;

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack = Vec::new();
        for c in s.bytes() {
            match (stack.last(), c) {
                (Some(&b'A'), b'B') | (Some(&b'C'), b'D') => {
                    stack.pop();
                }
                _ => {
                    stack.push(c);
                }
            }
        }
        stack.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_length() {
        let s = "ABFCACDB"; // cspell: disable-line
        let result = 2;
        assert_eq!(Solution::min_length(s.to_string()), result);
    }

    #[test]
    fn test_min_length_2() {
        let s = "ACBBD"; // cspell: disable-line
        let result = 5;
        assert_eq!(Solution::min_length(s.to_string()), result);
    }
}
