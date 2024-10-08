use crate::solutions::Solution;

impl Solution {
    pub fn min_swaps_3(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let ones = nums.iter().sum::<i32>() as usize;

        if ones == 0 || ones == n {
            return 0;
        }

        let mut current_ones = nums.iter().take(ones).sum::<i32>();
        let mut max_ones = current_ones;

        for i in 0..n {
            current_ones -= nums[i];
            current_ones += nums[(i + ones) % n];
            max_ones = max_ones.max(current_ones);
        }

        ones as i32 - max_ones
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_swaps() {
        let nums = vec![0, 1, 0, 1, 1, 0, 0];
        assert_eq!(Solution::min_swaps_3(nums), 1);
    }

    #[test]
    fn test_min_swaps_2() {
        let nums = vec![0, 1, 1, 1, 0, 0, 1, 1, 0];
        assert_eq!(Solution::min_swaps_3(nums), 2);
    }

    #[test]
    fn test_min_swaps_3() {
        let nums = vec![1, 1, 0, 0, 1];
        assert_eq!(Solution::min_swaps_3(nums), 0);
    }

    #[test]
    fn test_min_swaps_4() {
        let nums = vec![1];
        assert_eq!(Solution::min_swaps_3(nums), 0);
    }

    #[test]
    fn test_min_swaps_5() {
        let nums = vec![0, 0, 0];
        assert_eq!(Solution::min_swaps_3(nums), 0);
    }

    #[test]
    fn test_min_swaps_6() {
        let nums = vec![1, 1, 1, 0, 0, 1, 0, 1, 1, 0];
        assert_eq!(Solution::min_swaps_3(nums), 1);
    }
}
