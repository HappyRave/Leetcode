use crate::solutions::Solution;

impl Solution {
    pub fn maximum_strong_pair_xor(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if (nums[i] - nums[j]).abs() > std::cmp::min(nums[i], nums[j]) {
                    continue;
                }
                result = result.max(nums[i] ^ nums[j]);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_strong_pair_xor() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = 7;
        assert_eq!(Solution::maximum_strong_pair_xor(nums), result);
    }

    #[test]
    fn test_maximum_strong_pair_xor_2() {
        let nums = vec![0];
        let result = 0;
        assert_eq!(Solution::maximum_strong_pair_xor(nums), result);
    }

    #[test]
    fn test_maximum_strong_pair_xor_3() {
        let nums = vec![2, 4];
        let result = 6;
        assert_eq!(Solution::maximum_strong_pair_xor(nums), result);
    }

    #[test]
    fn test_maximum_strong_pair_xor_4() {
        let nums = vec![10, 100];
        let result = 0;
        assert_eq!(Solution::maximum_strong_pair_xor(nums), result);
    }

    #[test]
    fn test_maximum_strong_pair_xor_5() {
        let nums = vec![5, 6, 25, 30];
        let result = 7;
        assert_eq!(Solution::maximum_strong_pair_xor(nums), result);
    }
}
