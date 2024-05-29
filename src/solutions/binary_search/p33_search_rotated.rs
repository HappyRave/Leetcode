use crate::solutions::Solution;

impl Solution {
    pub fn search_rotated(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] == target {
                return mid;
            }
            if nums[left as usize] <= nums[mid as usize] {
                if nums[left as usize] <= target && target < nums[mid as usize] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else if nums[mid as usize] < target && target <= nums[right as usize] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_rotated() {
        assert_eq!(Solution::search_rotated(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn test_search_rotated_2() {
        assert_eq!(Solution::search_rotated(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }

    #[test]
    fn test_search_rotated_3() {
        assert_eq!(Solution::search_rotated(vec![1], 0), -1);
    }
}
