use super::Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut reversed = 0;
        let mut temp = x;
        while temp != 0 {
            reversed = reversed * 10 + temp % 10;
            temp /= 10;
        }
        x == reversed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
