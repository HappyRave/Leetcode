use crate::solutions::Solution;

impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut result = 0;
        for i in 1..nums.len() {
            if nums[i] <= nums[i - 1] {
                result += nums[i - 1] - nums[i] + 1;
                nums[i] = nums[i - 1] + 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_increment_for_unique() {
        let nums = vec![1, 2, 2];
        let result = 1;
        assert_eq!(Solution::min_increment_for_unique(nums), result);
    }

    #[test]
    fn test_min_increment_for_unique_2() {
        let nums = vec![3, 2, 1, 2, 1, 7];
        let result = 6;
        assert_eq!(Solution::min_increment_for_unique(nums), result);
    }

    #[test]
    fn test_min_increment_for_unique_3() {
        let nums = vec![1, 2, 2, 3, 3, 4];
        let result = 6;
        assert_eq!(Solution::min_increment_for_unique(nums), result);
    }
}
