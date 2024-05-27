use crate::solutions::Solution;

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect::<Vec<_>>();
        let size = vec![1; n];
        UnionFind { parent, size }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> usize {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            if self.size[root_x] < self.size[root_y] {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
                root_y
            } else {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
                root_x
            }
        } else {
            root_x
        }
    }
}

impl Solution {
    pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut grid = grid;
        let mut union_find = UnionFind::new(rows * cols + 1);
        let top = rows * cols;

        // Mark the bricks to be hit
        for hit in &hits {
            let (hit_row, hit_col) = (hit[0] as usize, hit[1] as usize);
            if grid[hit_row][hit_col] == 1 {
                grid[hit_row][hit_col] = -1;
            }
        }

        // Union the bricks
        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == 1 {
                    let index = row * cols + col;
                    if row == 0 {
                        union_find.union(col, top);
                    }
                    if row > 0 && grid[row - 1][col] == 1 {
                        union_find.union(index, (row - 1) * cols + col);
                    }
                    if col > 0 && grid[row][col - 1] == 1 {
                        union_find.union(index, row * cols + col - 1);
                    }
                }
            }
        }

        let mut results = vec![0; hits.len()];
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        // Reverse the hits
        for k in (0..hits.len()).rev() {
            let (hit_row, hit_col) = (hits[k][0] as usize, hits[k][1] as usize);

            if grid[hit_row][hit_col] == -1 {
                grid[hit_row][hit_col] = 1;
                let index = hit_row * cols + hit_col;

                let parent = union_find.find(top);
                let previous_size = union_find.size[parent];

                if hit_row == 0 {
                    union_find.union(hit_col, top);
                }

                // Union the hit brick with its neighbors
                for direction in &directions {
                    let (new_row, new_col) = (
                        hit_row as isize + direction.0,
                        hit_col as isize + direction.1,
                    );

                    if new_row >= 0
                        && new_row < rows as isize
                        && new_col >= 0
                        && new_col < cols as isize
                    {
                        let (new_row, new_col) = (new_row as usize, new_col as usize);
                        if grid[new_row][new_col] == 1 {
                            union_find.union(index, new_row * cols + new_col);
                        }
                    }
                }

                let parent = union_find.find(top);
                let size = union_find.size[parent];
                results[k] = 0.max(size as i32 - previous_size as i32 - 1);
            }
        }

        results
    }
}

struct SolutionSlow;
impl SolutionSlow {
    pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (0, -1), (1, 0)];
        fn is_stable(
            grid: &[Vec<i32>],
            rows_count: usize,
            cols_count: usize,
            i: usize,
            j: usize,
            visited: &mut Vec<[usize; 2]>,
        ) -> bool {
            if grid[i][j] == 0 || visited.contains(&[i, j]) {
                return false;
            }
            if i == 0 {
                return true;
            }

            visited.push([i, j]);

            for (di, dj) in DIRECTIONS.iter() {
                let new_i = (i as isize) + di;
                let new_j = (j as isize) + dj;
                if new_i >= 0
                    && new_i < (rows_count as isize)
                    && new_j >= 0
                    && new_j < (cols_count as isize)
                    && is_stable(
                        grid,
                        rows_count,
                        cols_count,
                        new_i as usize,
                        new_j as usize,
                        visited,
                    )
                {
                    return true;
                }
            }
            false
        }

        fn erase_bricks(
            grid: &mut [Vec<i32>],
            rows_count: usize,
            cols_count: usize,
            i: usize,
            j: usize,
        ) -> i32 {
            if i >= rows_count || j >= cols_count || grid[i][j] == 0 {
                return 0;
            }
            grid[i][j] = 0;

            let mut count = 0;
            let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (di, dj) in directions.iter() {
                let new_i = (i as isize) + di;
                let new_j = (j as isize) + dj;
                if new_i >= 0
                    && new_i < (rows_count as isize)
                    && new_j >= 0
                    && new_j < (cols_count as isize)
                {
                    let mut visited: Vec<[usize; 2]> = Vec::new();
                    if !is_stable(
                        grid,
                        rows_count,
                        cols_count,
                        new_i as usize,
                        new_j as usize,
                        &mut visited,
                    ) {
                        visited.iter().for_each(|[i, j]| {
                            grid[*i][*j] = 0;
                            count += 1;
                        });
                    }
                }
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
                    erase_bricks(&mut grid, rows_count, cols_count, i, j)
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

    #[test]
    fn test_hit_bricks_3() {
        assert_eq!(
            Solution::hit_bricks(
                vec![vec![1, 1, 1], vec![0, 1, 0], vec![0, 0, 0]],
                vec![vec![0, 2], vec![2, 0], vec![0, 1], vec![1, 2]]
            ),
            vec![0, 0, 1, 0]
        );
    }
}
