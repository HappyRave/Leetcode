use crate::solutions::Solution;

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left < right {
            let mid = left + (right - left) / 2;
            let neighbor = 1 - 2 * (mid % 2);
            if nums[mid as usize] == nums[(mid + neighbor) as usize] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        nums[left as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_non_duplicate() {
        let nums = vec![1, 1, 2, 3, 3, 4, 4, 8, 8];
        assert_eq!(Solution::single_non_duplicate(nums), 2);
    }

    #[test]
    fn test_single_non_duplicate_2() {
        let nums = vec![3, 3, 7, 7, 10, 11, 11];
        assert_eq!(Solution::single_non_duplicate(nums), 10);
    }

    #[test]
    fn test_single_non_duplicate_3() {
        let nums = vec![1, 1, 2, 3, 3];
        assert_eq!(Solution::single_non_duplicate(nums), 2);
    }
}
