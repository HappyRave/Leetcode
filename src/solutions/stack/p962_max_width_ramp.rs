use crate::solutions::Solution;

impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut res = 0;
        for i in 0..nums.len() {
            if stack.is_empty() || nums[stack[stack.len() - 1]] > nums[i] {
                stack.push(i);
            }
        }
        for i in (0..nums.len()).rev() {
            while !stack.is_empty() && nums[stack[stack.len() - 1]] <= nums[i] {
                res = res.max(i - stack.pop().unwrap());
            }
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_width_ramp() {
        assert_eq!(Solution::max_width_ramp(vec![6, 0, 8, 2, 1, 5]), 4);
    }

    #[test]
    fn test_max_width_ramp_2() {
        assert_eq!(
            Solution::max_width_ramp(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1]),
            7
        );
    }
}
