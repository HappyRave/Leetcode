use crate::solutions::Solution;

impl Solution {
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut left = 0;
        let mut right = nums[nums.len() - 1] - nums[0];
        while left < right {
            let mid = left + (right - left) / 2;
            let mut count = 0;
            let mut start = 0;
            for end in 0..nums.len() {
                while nums[end] - nums[start] > mid {
                    start += 1;
                }
                count += end - start;
            }
            if count < k as usize {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_distance_pair() {
        let nums = vec![1, 3, 1];
        let k = 1;
        let result = 0;
        assert_eq!(Solution::smallest_distance_pair(nums, k), result);
    }

    #[test]
    fn test_smallest_distance_pair_2() {
        let nums = vec![1, 1, 1];
        let k = 2;
        let result = 0;
        assert_eq!(Solution::smallest_distance_pair(nums, k), result);
    }

    #[test]
    fn test_smallest_distance_pair_3() {
        let nums = vec![1, 6, 1];
        let k = 3;
        let result = 5;
        assert_eq!(Solution::smallest_distance_pair(nums, k), result);
    }
}
