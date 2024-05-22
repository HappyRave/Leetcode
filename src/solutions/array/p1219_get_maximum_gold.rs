use crate::solutions::Solution;

impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let mut max_gold = 0;
        let mut grid = grid;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != 0 {
                    max_gold = max_gold.max(Self::dfs(&mut grid, i, j));
                }
            }
        }
        max_gold
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        let mut max_gold = 0;
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let current_gold = grid[i][j];
        grid[i][j] = 0;
        for (dx, dy) in directions {
            let x = i as i32 + dx;
            let y = j as i32 + dy;
            if x >= 0
                && x < grid.len() as i32
                && y >= 0
                && y < grid[0].len() as i32
                && grid[x as usize][y as usize] != 0
            {
                max_gold = max_gold.max(Self::dfs(grid, x as usize, y as usize));
            }
        }
        grid[i][j] = current_gold;
        max_gold + current_gold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_maximum_gold() {
        assert_eq!(Solution::get_maximum_gold(vec![vec![0]]), 0);
    }

    #[test]
    fn test_get_maximum_gold_2() {
        assert_eq!(
            Solution::get_maximum_gold(vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]]),
            24
        );
    }
}
