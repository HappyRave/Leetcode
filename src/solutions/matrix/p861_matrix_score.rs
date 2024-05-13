use crate::solutions::Solution;

impl Solution {
    pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;

        // leading bit to 1
        for i in 0..grid.len() {
            if grid[i][0] == 0 {
                for j in 0..grid[0].len() {
                    grid[i][j] = 1 - grid[i][j];
                }
            }
        }
        // max config for each column
        for j in 1..grid[0].len() {
            let mut count = 0;
            (0..grid.len()).for_each(|i| {
                count += grid[i][j];
            });
            if count * 2 < grid.len() as i32 {
                (0..grid.len()).for_each(|i| {
                    grid[i][j] = 1 - grid[i][j];
                });
            }
        }

        let mut result = 0;
        for i in 0..grid.len() {
            let mut row = 0;
            for j in 0..grid[0].len() {
                row = row * 2 + grid[i][j];
            }
            result += row;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_score() {
        let grid = vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]];
        let result = Solution::matrix_score(grid);
        assert_eq!(result, 39);
    }

    #[test]
    fn test_matrix_score_2() {
        let grid = vec![vec![0]];
        let result = Solution::matrix_score(grid);
        assert_eq!(result, 1);
    }
}
