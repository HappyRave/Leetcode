use crate::solutions::Solution;

impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut s = s
            .chars()
            .map(|c| (c as i32 - 'a' as i32 + 1).to_string())
            .collect::<String>();
        let mut res = 0;
        for _ in 0..k {
            res = s.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>();
            s = res.to_string();
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lucky() {
        let s = "iiii".to_string();
        let k = 1;
        let res = 36;
        assert_eq!(Solution::get_lucky(s, k), res);
    }

    #[test]
    fn test_get_lucky_2() {
        let s = "leetcode".to_string();
        let k = 2;
        let res = 6;
        assert_eq!(Solution::get_lucky(s, k), res);
    }

    #[test]
    fn test_get_lucky_3() {
        let s = "zbax".to_string(); // cspell: disable-line
        let k = 2;
        let res = 8;
        assert_eq!(Solution::get_lucky(s, k), res);
    }
}
