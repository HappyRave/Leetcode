use crate::solutions::Solution;

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());
        if n < 3 || m < 3 {
            return 0;
        }
        let mut prefix_sum_rows = vec![vec![0; m + 1]; n + 1];
        let mut prefix_sum_cols = vec![vec![0; m + 1]; n + 1];
        for i in 0..n {
            for j in 0..m {
                prefix_sum_rows[i][j + 1] = prefix_sum_rows[i][j] + grid[i][j];
                prefix_sum_cols[i + 1][j] = prefix_sum_cols[i][j] + grid[i][j];
            }
        }
        fn is_magic_square(
            i: usize,
            j: usize,
            grid: &[Vec<i32>],
            prefix_sum_rows: &[Vec<i32>],
            prefix_sum_cols: &[Vec<i32>],
        ) -> bool {
            for k in 0..3 {
                if prefix_sum_rows[i + k][j + 3] - prefix_sum_rows[i + k][j] != 15 {
                    return false;
                }
                if prefix_sum_cols[i + 3][j + k] - prefix_sum_cols[i][j + k] != 15 {
                    return false;
                }
            }
            if grid[i][j] + grid[i + 1][j + 1] + grid[i + 2][j + 2] != 15 {
                return false;
            }
            if grid[i][j + 2] + grid[i + 1][j + 1] + grid[i + 2][j] != 15 {
                return false;
            }
            for k in 0..3 {
                for l in 0..3 {
                    if grid[i + k][j + l] < 1 || grid[i + k][j + l] > 9 {
                        return false;
                    }
                    if (k, l) != (0, 0) && grid[i + k][j + l] == grid[i][j] {
                        return false;
                    }
                }
            }
            true
        }

        let mut squares = 0;
        for i in 0..n - 2 {
            for j in 0..m - 2 {
                if is_magic_square(i, j, &grid, &prefix_sum_rows, &prefix_sum_cols) {
                    squares += 1;
                }
            }
        }
        squares
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::ArrayStringExt;

    use super::*;

    #[test]
    fn test_num_magic_squares_inside() {
        let grid = "[[4,3,8,4],[9,5,1,9],[2,7,6,2]]".to_matrix();
        assert_eq!(Solution::num_magic_squares_inside(grid), 1);
    }

    #[test]
    fn test_num_magic_squares_inside_2() {
        let grid = "[[8]]".to_matrix();
        assert_eq!(Solution::num_magic_squares_inside(grid), 0);
    }

    #[test]
    fn test_num_magic_squares_inside_3() {
        let grid = "[[5,5,5],[5,5,5],[5,5,5]]".to_matrix();
        assert_eq!(Solution::num_magic_squares_inside(grid), 0);
    }

    #[test]
    fn test_num_magic_squares_inside_4() {
        let grid = "[[1,8,6],[10,5,0],[4,2,9]]".to_matrix();
        assert_eq!(Solution::num_magic_squares_inside(grid), 0);
    }
}
