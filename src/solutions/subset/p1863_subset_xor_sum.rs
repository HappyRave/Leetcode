use crate::solutions::Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for &num in &nums {
            res |= num;
        }
        res << (nums.len() - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subset_xor_sum() {
        let nums = vec![1, 3];
        assert_eq!(Solution::subset_xor_sum(nums), 6);
    }

    #[test]
    fn test_subset_xor_sum_2() {
        let nums = vec![5, 1, 6];
        assert_eq!(Solution::subset_xor_sum(nums), 28);
    }

    #[test]
    fn test_subset_xor_sum_3() {
        let nums = vec![3, 4, 5, 6, 7, 8];
        assert_eq!(Solution::subset_xor_sum(nums), 480);
    }
}
