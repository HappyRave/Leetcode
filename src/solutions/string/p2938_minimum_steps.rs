use crate::solutions::Solution;

impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        s.bytes()
            .enumerate()
            .filter(|(_, c)| *c == b'0')
            .enumerate()
            .map(|(i, (j, _))| (j - i) as i64)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_steps() {
        let s = "101".to_string();
        let res = 1;
        assert_eq!(Solution::minimum_steps(s), res);
    }

    #[test]
    fn test_minimum_steps_2() {
        let s = "100".to_string();
        let res = 2;
        assert_eq!(Solution::minimum_steps(s), res);
    }

    #[test]
    fn test_minimum_steps_3() {
        let s = "0111".to_string();
        let res = 0;
        assert_eq!(Solution::minimum_steps(s), res);
    }
}
