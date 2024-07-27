use crate::solutions::Solution;

impl Solution {
    pub fn monotone_increasing_digits(mut n: i32) -> i32 {
        let (mut prev, mut result, mut multiplier) = (9, 0, 1);
        while n > 0 {
            let digit = n % 10;
            n /= 10;
            if digit <= prev {
                result += digit * multiplier;
                prev = digit;
            } else {
                result = digit * multiplier - 1;
                prev = digit - 1;
            }
            multiplier *= 10;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monotone_increasing_digits() {
        assert_eq!(Solution::monotone_increasing_digits(10), 9);
    }

    #[test]
    fn test_monotone_increasing_digits_2() {
        assert_eq!(Solution::monotone_increasing_digits(1234), 1234);
    }
    #[test]
    fn test_monotone_increasing_digits_3() {
        assert_eq!(Solution::monotone_increasing_digits(332), 299);
    }
}
