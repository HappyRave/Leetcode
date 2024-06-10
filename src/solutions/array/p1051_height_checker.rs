use crate::solutions::Solution;

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut freq = [0; 101];
        for &height in &heights {
            freq[height as usize] += 1;
        }
        let mut result = 0;
        let mut j = 0;
        for &height in &heights {
            while freq[j] == 0 {
                j += 1;
            }
            if j != height as usize {
                result += 1;
            }
            freq[j] -= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height_checker() {
        let heights = vec![1, 1, 4, 2, 1, 3];
        let result = 3;
        assert_eq!(Solution::height_checker(heights), result);
    }

    #[test]
    fn test_height_checker_2() {
        let heights = vec![5, 1, 2, 3, 4];
        let result = 5;
        assert_eq!(Solution::height_checker(heights), result);
    }

    #[test]
    fn test_height_checker_3() {
        let heights = vec![1, 2, 3, 4, 5];
        let result = 0;
        assert_eq!(Solution::height_checker(heights), result);
    }
}
