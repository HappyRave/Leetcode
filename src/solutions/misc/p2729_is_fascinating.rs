use crate::solutions::Solution;

impl Solution {
    pub fn is_fascinating(n: i32) -> bool {
        if !(100..=333).contains(&n) {
            return false;
        }
        let mut digits = [0; 10];
        let mut n = n * 1_000_000 + n * 2_000 + n * 3;
        while n > 0 {
            digits[(n % 10) as usize] += 1;
            n /= 10;
        }
        #[allow(clippy::needless_range_loop)]
        for i in 1..10 {
            if digits[i] != 1 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_fascinating() {
        assert!(Solution::is_fascinating(192));
    }

    #[test]
    fn test_is_fascinating_2() {
        assert!(!Solution::is_fascinating(100));
    }

    #[test]
    fn test_is_fascinating_3() {
        assert!(!Solution::is_fascinating(853));
    }

    #[test]
    fn test_is_fascinating_4() {
        assert!(!Solution::is_fascinating(333));
    }
}
