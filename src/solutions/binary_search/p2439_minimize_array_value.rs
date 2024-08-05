use crate::solutions::Solution;

impl Solution {
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        let mut sum: usize = 0;
        nums.into_iter()
            .enumerate()
            .map(|(i, n)| {
                sum += n as usize;
                (sum + i) / (i + 1)
            })
            .max()
            .unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimize_array_value() {
        assert_eq!(Solution::minimize_array_value(vec![3, 7, 1, 6]), 5);
    }

    #[test]
    fn test_minimize_array_value_2() {
        assert_eq!(Solution::minimize_array_value(vec![10, 1]), 10);
    }
}
