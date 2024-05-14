use crate::solutions::Solution;

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        const DIRECTIONS: [(i32, i32); 8] = [
            (1, 1),
            (1, 0),
            (1, -1),
            (0, 1),
            (0, -1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ];
        (1..grid.len() - 1).for_each(|i| {
            let mut row = vec![];
            (1..grid[0].len() - 1).for_each(|j| {
                let mut max = grid[i][j];

                for (dx, dy) in DIRECTIONS {
                    let x = i as i32 + dx;
                    let y = j as i32 + dy;
                    max = max.max(grid[x as usize][y as usize]);
                }
                row.push(max);
            });
            result.push(row);
        });
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_local() {
        assert_eq!(
            Solution::largest_local(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]),
            vec![vec![0]]
        );
    }

    #[test]
    fn test_largest_local_2() {
        assert_eq!(
            Solution::largest_local(vec![
                vec![9, 9, 8, 1],
                vec![5, 6, 2, 6],
                vec![8, 2, 6, 4],
                vec![6, 2, 2, 2]
            ]),
            vec![vec![9, 9], vec![8, 6]]
        );
    }
}
