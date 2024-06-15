use crate::solutions::Solution;

impl Solution {
    pub fn sort_colors(nums: &mut [i32]) {
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut i = 0;
        while i <= right {
            if nums[i] == 0 {
                nums[i] = nums[left];
                nums[left] = 0;
                left += 1;
                i += 1;
            } else if nums[i] == 2 && right > 0 {
                nums[i] = nums[right];
                nums[right] = 2;
                right -= 1;
            } else {
                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_colors() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_sort_colors_2() {
        let mut nums = vec![2, 0, 1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }

    #[test]
    fn test_sort_colors_3() {
        let mut nums = vec![0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn test_sort_colors_4() {
        let mut nums = vec![1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![1]);
    }

    #[test]
    fn test_sort_colors_5() {
        let mut nums = vec![2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![2]);
    }
}
