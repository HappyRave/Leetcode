use crate::solutions::Solution;

impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid1.len(), grid1[0].len());
        let mut uf = UnionFind::new(m * n);
        for i in 0..m {
            for j in 0..n {
                if grid2[i][j] == 1 {
                    let index = i * n + j;
                    if j + 1 < n && grid2[i][j + 1] == 1 {
                        uf.union(index, index + 1);
                    }
                    if i + 1 < m && grid2[i + 1][j] == 1 {
                        uf.union(index, index + n);
                    }
                }
            }
        }

        let mut invalid_islands = std::collections::HashSet::new();
        let mut islands = std::collections::HashSet::new();
        for i in 0..m {
            for j in 0..n {
                if grid2[i][j] == 1 {
                    let root = uf.find(i * n + j);
                    islands.insert(root);
                    if grid1[i][j] == 0 {
                        invalid_islands.insert(root);
                    }
                }
            }
        }

        (islands.len() - invalid_islands.len()) as i32
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: usize,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            size,
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while x != self.parent[x] {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            self.parent[root_x] = root_y;
            self.size -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::ArrayStringExt;

    use super::*;

    #[test]
    fn test_count_sub_islands() {
        let grid1 = "[[1,1,1,0,0],[0,1,1,1,1],[0,0,0,0,0],[1,0,0,0,0],[1,1,0,1,1]]".to_matrix();
        let grid2 = "[[1,1,1,0,0],[0,0,1,1,1],[0,1,0,0,0],[1,0,1,1,0],[0,1,0,1,0]]".to_matrix();
        assert_eq!(Solution::count_sub_islands(grid1, grid2), 3);
    }

    #[test]
    fn test_count_sub_islands_2() {
        let grid1 = "[[1,0,1,0,1],[1,1,1,1,1],[0,0,0,0,0],[1,1,1,1,1],[1,0,1,0,1]]".to_matrix();
        let grid2 = "[[0,0,0,0,0],[1,1,1,1,1],[0,1,0,1,0],[0,1,0,1,0],[1,0,0,0,1]]".to_matrix();
        assert_eq!(Solution::count_sub_islands(grid1, grid2), 2);
    }
}
