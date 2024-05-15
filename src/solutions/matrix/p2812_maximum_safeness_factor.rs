use crate::solutions::Solution;

impl Solution {
    pub fn maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if n < 2 {
            return 0;
        }
        if (grid[0][0] == 1) || (grid[n - 1][n - 1] == 1) {
            return 0;
        }

        let mut queue = std::collections::VecDeque::new();
        (0..n).for_each(|i| {
            (0..n).for_each(|j| {
                if grid[i][j] == 1 {
                    queue.push_back((i, j, 1));
                    grid[i][j] = 0;
                } else {
                    grid[i][j] = -1;
                }
            });
        });

        const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        while let Some((i, j, safety)) = queue.pop_front() {
            for (dx, dy) in DIRECTIONS.iter() {
                let x = i as i32 + dx;
                let y = j as i32 + dy;
                if (x >= 0) && (x < n as i32) && (y >= 0) && (y < n as i32) {
                    let x = x as usize;
                    let y = y as usize;
                    if grid[x][y] == -1 {
                        grid[x][y] = safety;
                        queue.push_back((x, y, safety + 1));
                    }
                }
            }
        }

        let mut min_safety = grid[0][0];
        queue.push_back((0, 0, grid[0][0]));
        while let Some((i, j, safety)) = queue.pop_front() {
            min_safety = min_safety.min(safety);
            if (i == n - 1) && (j == n - 1) {
                break;
            }
            for (dx, dy) in DIRECTIONS.iter() {
                let x = i as i32 + dx;
                let y = j as i32 + dy;
                if (x >= 0) && (x < n as i32) && (y >= 0) && (y < n as i32) {
                    let x = x as usize;
                    let y = y as usize;
                    if grid[x][y] != -1 {
                        let new_safety = grid[x][y];
                        grid[x][y] = -1;
                        if new_safety < min_safety {
                            queue.push_back((x, y, new_safety));
                        } else {
                            queue.push_front((x, y, new_safety));
                        }
                    }
                }
            }
        }
        min_safety
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_safeness_factor() {
        assert_eq!(
            Solution::maximum_safeness_factor(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]]),
            0
        );
    }

    #[test]
    fn test_maximum_safeness_factor_2() {
        assert_eq!(
            Solution::maximum_safeness_factor(vec![vec![0, 0, 1], vec![0, 0, 0], vec![0, 0, 0]]),
            2
        );
    }

    #[test]
    fn test_maximum_safeness_factor_3() {
        assert_eq!(
            Solution::maximum_safeness_factor(vec![
                vec![0, 0, 0, 1],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![1, 0, 0, 0]
            ]),
            2
        );
    }

    #[test]
    fn test_maximum_safeness_factor_4() {
        assert_eq!(
            Solution::maximum_safeness_factor(vec![vec![0, 1], vec![0, 0],]),
            1
        );
    }

    #[test]
    fn test_maximum_safeness_factor_5() {
        assert_eq!(
            Solution::maximum_safeness_factor(vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
                vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
                vec![1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0]
            ]),
            7
        );
    }
}
