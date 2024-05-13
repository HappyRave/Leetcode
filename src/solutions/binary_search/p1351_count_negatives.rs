use crate::solutions::Solution;

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        for row in &grid {
            let mut left = 0;
            let mut right = row.len() as i32 - 1;
            while left <= right {
                let mid = left + (right - left) / 2;
                if row[mid as usize] < 0 {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            result += row.len() as i32 - left;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_negatives() {
        let grid = vec![
            vec![4, 3, 2, -1],
            vec![3, 2, 1, -1],
            vec![1, 1, -1, -2],
            vec![-1, -1, -2, -3],
        ];
        let result = Solution::count_negatives(grid);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_count_negatives_2() {
        let grid = vec![vec![3, 2], vec![1, 0]];
        let result = Solution::count_negatives(grid);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_count_negatives_3() {
        let grid = vec![vec![1, -1], vec![-1, -1]];
        let result = Solution::count_negatives(grid);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_count_negatives_4() {
        let grid = vec![vec![-1]];
        let result = Solution::count_negatives(grid);
        assert_eq!(result, 1);
    }
}
