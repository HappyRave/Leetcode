use crate::solutions::Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut left, mut right) = (0, m * n - 1);
        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_val = matrix[mid / n][mid % n];
            match mid_val.cmp(&target) {
                std::cmp::Ordering::Less => left = mid + 1,
                std::cmp::Ordering::Greater => {
                    right = if mid > 0 { mid - 1 } else { return false }
                }
                std::cmp::Ordering::Equal => return true,
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_matrix() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert!(Solution::search_matrix(matrix, 3));
    }

    #[test]
    fn test_search_matrix_2() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert!(!Solution::search_matrix(matrix, 13));
    }

    #[test]
    fn test_search_matrix_3() {
        let matrix = vec![vec![1]];
        assert!(!Solution::search_matrix(matrix, 0));
    }

    #[test]
    fn test_search_matrix_4() {
        let matrix = vec![vec![1]];
        assert!(Solution::search_matrix(matrix, 1));
    }

    #[test]
    fn test_search_matrix_5() {
        let matrix = vec![vec![1, 1]];
        assert!(!Solution::search_matrix(matrix, 0));
    }
}
