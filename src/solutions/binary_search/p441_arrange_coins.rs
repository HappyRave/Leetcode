use crate::solutions::Solution;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        while left <= right {
            let mid = left + (right - left) / 2;
            let guess = mid as i64 * (mid as i64 + 1) / 2;
            if guess == n as i64 {
                return mid;
            }
            if guess < n as i64 {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrange_coins() {
        assert_eq!(Solution::arrange_coins(5), 2);
    }

    #[test]
    fn test_arrange_coins_2() {
        assert_eq!(Solution::arrange_coins(8), 3);
    }
}
