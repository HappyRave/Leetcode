use crate::solutions::Solution;

impl Solution {
    pub fn longest_subarray_2(nums: Vec<i32>) -> i32 {
        let mut max_val = nums[0];
        let mut max_len = 1;
        let mut current_len = 1;
        for &num in nums.iter().skip(1) {
            match num.cmp(&max_val) {
                std::cmp::Ordering::Equal => {
                    current_len += 1;
                    max_len = max_len.max(current_len);
                }
                std::cmp::Ordering::Greater => {
                    max_val = num;
                    current_len = 1;
                    max_len = 1;
                }
                std::cmp::Ordering::Less => {
                    current_len = 0;
                }
            }
        }
        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_subarray() {
        assert_eq!(Solution::longest_subarray_2(vec![1, 2, 3, 3, 2, 2]), 2);
    }

    #[test]
    fn test_longest_subarray_2() {
        assert_eq!(Solution::longest_subarray_2(vec![1, 2, 3, 4]), 1);
    }
}
