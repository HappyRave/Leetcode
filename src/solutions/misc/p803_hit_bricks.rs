use crate::solutions::Solution;

impl Solution {
    pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        fn is_stable(
            grid: &Vec<Vec<i32>>,
            rows_count: usize,
            cols_count: usize,
            i: usize,
            j: usize,
        ) -> bool {
            if i == 0 {
                return true;
            }
            if j > 0 && grid[i][j - 1] == 1 {
                return true;
            }
            if j < cols_count - 1 && grid[i][j + 1] == 1 {
                return true;
            }
            if grid[i - 1][j] == 1 {
                return true;
            }
            if i < rows_count - 1 && grid[i + 1][j] == 1 {
                return true;
            }

            false
        }

        fn erase_bricks(
            grid: &mut Vec<Vec<i32>>,
            rows_count: usize,
            cols_count: usize,
            i: usize,
            j: usize,
            test_stability: bool,
        ) -> i32 {
            if i >= rows_count || j >= cols_count || grid[i][j] == 0 {
                return 0;
            }
            if test_stability && is_stable(grid, rows_count, cols_count, i, j) {
                return 0;
            }
            grid[i][j] = 0;
            let mut count = if test_stability { 1 } else { 0 };
            if j > 0 {
                count += erase_bricks(grid, rows_count, cols_count, i, j - 1, true);
            }
            if j < cols_count - 1 {
                count += erase_bricks(grid, rows_count, cols_count, i, j + 1, true);
            }
            if i > 0 {
                count += erase_bricks(grid, rows_count, cols_count, i - 1, j, true);
            }
            if i < rows_count - 1 {
                count += erase_bricks(grid, rows_count, cols_count, i + 1, j, true);
            }
            count
        }

        let rows_count = grid.len();
        let cols_count = grid[0].len();
        let mut grid = grid;
        hits.iter()
            .map(|hit| {
                let i = hit[0] as usize;
                let j = hit[1] as usize;
                if grid[i][j] == 0 {
                    0
                } else {
                    erase_bricks(&mut grid, rows_count, cols_count, i, j, false)
                }
            })
            .collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_bricks() {
        assert_eq!(
            Solution::hit_bricks(vec![vec![1, 0, 0, 0], vec![1, 1, 1, 0]], vec![vec![1, 0]]),
            vec![2]
        );
    }

    #[test]
    fn test_hit_bricks_2() {
        assert_eq!(
            Solution::hit_bricks(
                vec![vec![1, 0, 0, 0], vec![1, 1, 0, 0]],
                vec![vec![1, 1], vec![1, 0]]
            ),
            vec![0, 0]
        );
    }
}
