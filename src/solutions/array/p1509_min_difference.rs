use crate::solutions::Solution;

impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        if n <= 3 {
            return 0;
        }
        let mut result = i32::MAX;
        for i in 0..4 {
            result = result.min(nums[n - 4 + i] - nums[i]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_difference() {
        assert_eq!(Solution::min_difference(vec![5, 3, 2, 4]), 0);
    }

    #[test]
    fn test_min_difference_2() {
        assert_eq!(Solution::min_difference(vec![1, 5, 0, 10, 14]), 1);
    }

    #[test]
    fn test_min_difference_3() {
        assert_eq!(Solution::min_difference(vec![3, 100, 20]), 0);
    }
}
