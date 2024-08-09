use crate::solutions::Solution;

impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());
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
            size: usize,
            grid: &[Vec<i32>],
            prefix_sum_rows: &[Vec<i32>],
            prefix_sum_cols: &[Vec<i32>],
        ) -> bool {
            let target = prefix_sum_rows[i][j + size] - prefix_sum_rows[i][j];
            for k in 0..size {
                if prefix_sum_rows[i + k][j + size] - prefix_sum_rows[i + k][j] != target {
                    return false;
                }
                if prefix_sum_cols[i + size][j + k] - prefix_sum_cols[i][j + k] != target {
                    return false;
                }
            }
            let (mut diag1, mut diag2) = (0, 0);
            for k in 0..size {
                diag1 += grid[i + k][j + k];
                diag2 += grid[i + k][j + size - k - 1];
            }
            if diag1 != target || diag2 != target {
                return false;
            }
            true
        }

        let mut largest = 1;
        let (mut row, mut col) = (0, 0);
        while row < n - largest {
            while col < m - largest {
                let mut size = largest + 1;
                while row + size <= n && col + size <= m {
                    if is_magic_square(row, col, size, &grid, &prefix_sum_rows, &prefix_sum_cols) {
                        largest = size;
                    }
                    size += 1;
                }
                col += 1;
            }
            row += 1;
            col = 0;
        }
        largest as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::ArrayStringExt;

    use super::*;

    #[test]
    fn test_largest_magic_square() {
        let grid = "[[7,1,4,5,6],[2,5,1,6,4],[1,5,4,3,2],[1,2,7,3,4]]".to_matrix();
        assert_eq!(Solution::largest_magic_square(grid), 3);
    }

    #[test]
    fn test_largest_magic_square_2() {
        let grid = "[[5,1,3,1],[9,3,3,1],[1,3,3,8]]".to_matrix();
        assert_eq!(Solution::largest_magic_square(grid), 2);
    }

    #[test]
    fn test_largest_magic_square_3() {
        let grid = "[[1]]".to_matrix();
        assert_eq!(Solution::largest_magic_square(grid), 1);
    }
}
