use crate::solutions::Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut count = [0; 128];
        for c in s.chars() {
            count[c as usize] += 1;
        }
        let mut result = 0;
        let mut has_odd = false;
        for c in count.iter() {
            result += c / 2 * 2;
            if c % 2 == 1 {
                has_odd = true;
            }
        }
        if has_odd {
            result += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        let s = "abccccdd".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_longest_palindrome_2() {
        let s = "a".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_longest_palindrome_3() {
        let s = "bb".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_longest_palindrome_4() {
        let s = "ab".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_longest_palindrome_5() {
        let s = "Aa".to_string();
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, 1);
    }
}
