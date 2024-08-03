use crate::solutions::Solution;

impl Solution {
    pub fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
        target.sort_unstable();
        arr.sort_unstable();
        target == arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_be_equal() {
        assert!(Solution::can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3]));
    }

    #[test]
    fn test_can_be_equal_2() {
        assert!(Solution::can_be_equal(vec![7], vec![7]));
    }

    #[test]
    fn test_can_be_equal_3() {
        assert!(!Solution::can_be_equal(vec![3, 7, 9], vec![3, 7, 11]));
    }
}
