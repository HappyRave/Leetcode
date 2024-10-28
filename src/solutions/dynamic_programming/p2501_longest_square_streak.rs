use crate::solutions::Solution;

impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let unique_nums = nums.into_iter().collect::<std::collections::HashSet<i32>>();
        let mut max_streak = -1;
        for &num in &unique_nums {
            let mut n = num;
            let mut streak = 1;
            while unique_nums.contains(&(n.saturating_mul(n))) {
                n = n.saturating_mul(n);
                streak += 1;
                max_streak = max_streak.max(streak);
            }
        }
        max_streak
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_square_streak() {
        let nums = vec![4, 3, 6, 16, 8, 2];
        let result = 3;
        assert_eq!(Solution::longest_square_streak(nums), result);
    }

    #[test]
    fn test_longest_square_streak_2() {
        let nums = vec![2, 3, 5, 6, 7];
        let result = -1;
        assert_eq!(Solution::longest_square_streak(nums), result);
    }
}
