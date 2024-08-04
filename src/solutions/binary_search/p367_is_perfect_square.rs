use crate::solutions::Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num < 2 {
            return true;
        }
        let mut left = 2;
        let mut right = num / 2;
        while left <= right {
            let x = left + (right - left) / 2;
            let guess = x as i64 * x as i64;
            if guess > i32::MAX as i64 {
                right = x - 1;
                continue;
            }
            let guess = guess as i32;
            if guess == num {
                return true;
            }
            if guess < num {
                left = x + 1;
            } else {
                right = x - 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_perfect_square() {
        let num = 16;
        assert!(Solution::is_perfect_square(num));
    }

    #[test]
    fn test_is_perfect_square_2() {
        let num = 14;
        assert!(!Solution::is_perfect_square(num));
    }

    #[test]
    fn test_is_perfect_square_3() {
        let num = 808201;
        assert!(Solution::is_perfect_square(num));
    }
}
