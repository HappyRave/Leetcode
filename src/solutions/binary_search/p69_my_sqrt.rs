use crate::solutions::Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x;
        }
        let mut left = 1;
        let mut right = x / 2;
        while left <= right {
            let mid = left + (right - left) / 2;
            let guess = mid as i64 * mid as i64;
            if guess > i32::MAX as i64 {
                right = mid - 1;
                continue;
            }
            let guess = guess as i32;
            if guess == x {
                return mid;
            }
            if guess < x {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        left - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_sqrt() {
        let x = 4;
        assert_eq!(Solution::my_sqrt(x), 2);
    }

    #[test]
    fn test_my_sqrt_2() {
        let x = 8;
        assert_eq!(Solution::my_sqrt(x), 2);
    }

    #[test]
    fn test_my_sqrt_3() {
        let x = 2;
        assert_eq!(Solution::my_sqrt(x), 1);
    }

    #[test]
    fn test_my_sqrt_4() {
        let x = 4;
        assert_eq!(Solution::my_sqrt(x), 2);
    }
}
