use crate::solutions::Solution;

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        if edges[0][0] == edges[1][0] || edges[0][0] == edges[1][1] {
            edges[0][0]
        } else {
            edges[0][1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_center() {
        assert_eq!(
            Solution::find_center(vec![vec![1, 2], vec![2, 3], vec![4, 2]]),
            2
        );
    }

    #[test]
    fn test_find_center_2() {
        assert_eq!(
            Solution::find_center(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]]),
            1
        );
    }
}
