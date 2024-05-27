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
        let m = grid.len();
        let n = grid[0].len();
        let mut grid = grid;
        let mut uf = UnionFind::new(m * n + 1);
        let top = m * n;

        for hit in &hits {
            grid[hit[0] as usize][hit[1] as usize] -= 1;
        }

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    if i == 0 {
                        uf.union(i * n + j, top);
                    }
                    if i > 0 && grid[i - 1][j] == 1 {
                        uf.union(i * n + j, (i - 1) * n + j);
                    }
                    if j > 0 && grid[i][j - 1] == 1 {
                        uf.union(i * n + j, i * n + j - 1);
                    }
                }
            }
        }

        let mut res = vec![0; hits.len()];
        let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        for k in (0..hits.len()).rev() {
            let i = hits[k][0] as usize;
            let j = hits[k][1] as usize;

            if grid[i][j] == 0 {
                grid[i][j] = 1;
                if i == 0 {
                    uf.union(i * n + j, top);
                }

                for dir in &dirs {
                    let x = i as isize + dir.0;
                    let y = j as isize + dir.1;

                    if x >= 0 && x < m as isize && y >= 0 && y < n as isize {
                        let x = x as usize;
                        let y = y as usize;
                        if grid[x][y] == 1 {
                            uf.union(i * n + j, x * n + y);
                        }
                    }
                }

                let size = uf.size.clone();
                let find = uf.find(i * n + j);
                res[k] = size[find] - 1;
            }
        }

        res.iter().map(|&x| x as i32).collect()
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
