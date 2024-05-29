use crate::solutions::Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] < nums[right as usize] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        nums[left as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
    }

    #[test]
    fn test_find_min_2() {
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    }

    #[test]
    fn test_find_min_3() {
        assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
    }
}
