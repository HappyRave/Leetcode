use crate::solutions::Solution;

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let mut n = n;
        let mut d = 2;
        let mut ans = 0;
        while n > 1 {
            while n % d == 0 {
                ans += d;
                n /= d;
            }
            d += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_steps() {
        assert_eq!(Solution::min_steps(3), 3);
    }

    #[test]
    fn test_min_steps_2() {
        assert_eq!(Solution::min_steps(1), 0);
    }

    #[test]
    fn test_min_steps_3() {
        assert_eq!(Solution::min_steps(10), 7);
    }
}
