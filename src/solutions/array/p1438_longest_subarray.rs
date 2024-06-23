use crate::solutions::Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut max_queue = std::collections::VecDeque::new();
        let mut min_queue = std::collections::VecDeque::new();
        let mut left = 0;
        let mut right = 0;
        let mut result = 0;
        while right < nums.len() {
            while !max_queue.is_empty() && nums[right] > nums[*max_queue.back().unwrap()] {
                max_queue.pop_back();
            }
            while !min_queue.is_empty() && nums[right] < nums[*min_queue.back().unwrap()] {
                min_queue.pop_back();
            }
            max_queue.push_back(right);
            min_queue.push_back(right);
            while nums[*max_queue.front().unwrap()] - nums[*min_queue.front().unwrap()] > limit {
                if left == *max_queue.front().unwrap() {
                    max_queue.pop_front();
                }
                if left == *min_queue.front().unwrap() {
                    min_queue.pop_front();
                }
                left += 1;
            }
            result = result.max(right - left + 1);
            right += 1;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_subarray() {
        let nums = vec![8, 2, 4, 7];
        let limit = 4;
        assert_eq!(Solution::longest_subarray(nums, limit), 2);
    }

    #[test]
    fn test_longest_subarray_2() {
        let nums = vec![10, 1, 2, 4, 7, 2];
        let limit = 5;
        assert_eq!(Solution::longest_subarray(nums, limit), 4);
    }

    #[test]
    fn test_longest_subarray_3() {
        let nums = vec![4, 2, 2, 2, 4, 4, 2, 2];
        let limit = 0;
        assert_eq!(Solution::longest_subarray(nums, limit), 3);
    }
}
