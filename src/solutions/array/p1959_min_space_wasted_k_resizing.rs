use crate::solutions::Solution;

impl Solution {
    pub fn min_space_wasted_k_resizing(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut dp = vec![0; n + 1];
        for iteration in 0..k + 1 {
            for (i, num) in nums.iter().enumerate().rev() {
                let mut waste = 0;
                let mut max = *num;
                let mut min = dp[i + 1];
                for j in (0..=i).rev() {
                    if nums[j] <= max {
                        waste += max - nums[j];
                    } else {
                        waste += (i - j) as i32 * (nums[j] - max);
                        max = nums[j];
                    }
                    if iteration > 0 {
                        min = std::cmp::min(min, waste + dp[j]);
                    } else {
                        min = waste;
                    }
                }
                dp[i + 1] = min;
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_space_wasted_k_resizing() {
        assert_eq!(Solution::min_space_wasted_k_resizing(vec![10, 20], 0), 10);
    }

    #[test]
    fn test_min_space_wasted_k_resizing_2() {
        assert_eq!(
            Solution::min_space_wasted_k_resizing(vec![10, 20, 30], 1),
            10
        );
    }

    #[test]
    fn test_min_space_wasted_k_resizing_3() {
        assert_eq!(
            Solution::min_space_wasted_k_resizing(vec![10, 20, 15, 30, 20], 2),
            15
        );
    }
}
