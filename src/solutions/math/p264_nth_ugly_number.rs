use crate::solutions::Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut dp = vec![1; n as usize];
        let (mut i2, mut i3, mut i5) = (0, 0, 0);
        let (mut next2, mut next3, mut next5) = (2, 3, 5);
        for i in 1..n as usize {
            dp[i] = next2.min(next3).min(next5);
            if dp[i] == next2 {
                i2 += 1;
                next2 = dp[i2] * 2;
            }
            if dp[i] == next3 {
                i3 += 1;
                next3 = dp[i3] * 3;
            }
            if dp[i] == next5 {
                i5 += 1;
                next5 = dp[i5] * 5;
            }
        }
        dp[n as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_ugly_number() {
        assert_eq!(Solution::nth_ugly_number(10), 12);
    }

    #[test]
    fn test_nth_ugly_number_2() {
        assert_eq!(Solution::nth_ugly_number(1), 1);
    }
}
