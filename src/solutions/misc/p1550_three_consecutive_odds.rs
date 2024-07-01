use crate::solutions::Solution;

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut count = 0;
        for i in arr {
            if i % 2 != 0 {
                count += 1;
                if count == 3 {
                    return true;
                }
            } else {
                count = 0;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_consecutive_odds() {
        let arr = vec![2, 6, 4, 1];
        assert!(!Solution::three_consecutive_odds(arr));
    }

    #[test]
    fn test_three_consecutive_odds_2() {
        let arr = vec![1, 2, 34, 3, 4, 5, 7, 23, 12];
        assert!(Solution::three_consecutive_odds(arr));
    }
}
