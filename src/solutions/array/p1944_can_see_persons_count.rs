use crate::solutions::Solution;

impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let len = heights.len();
        let mut stack: Vec<usize> = Vec::new();
        let mut result = vec![0; len];
        for i in 0..len {
            while let Some(&top) = stack.last() {
                if heights[top] < heights[i] {
                    result[top] += 1;
                    stack.pop();
                } else {
                    break;
                }
            }
            if let Some(&top) = stack.last() {
                result[top] += 1;
            }
            stack.push(i);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_see_persons_count() {
        assert_eq!(
            Solution::can_see_persons_count(vec![10, 6, 8, 5, 11, 9]),
            vec![3, 1, 2, 1, 1, 0]
        );
    }

    #[test]
    fn test_can_see_persons_count_2() {
        assert_eq!(
            Solution::can_see_persons_count(vec![5, 1, 2, 3, 10]),
            vec![4, 1, 1, 1, 0]
        );
    }
}
