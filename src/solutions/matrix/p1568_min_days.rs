use crate::solutions::Solution;

impl Solution {
    pub fn min_days_island(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let ones = (0..m * n)
            .filter(|&i| grid[i / n][i % n] == 1)
            .collect::<Vec<_>>();
        if ones.len() <= 1 {
            return ones.len() as i32;
        }
        fn is_disconnected(
            grid: &[Vec<i32>],
            ones: &[usize],
            skip: Option<usize>,
            n: usize,
        ) -> bool {
            let mut uf = UnionFind::new(ones.len());
            for (i, one) in ones.iter().enumerate() {
                if Some(*one) == skip {
                    continue;
                }
                if one % n > 0 && grid[one / n][one % n - 1] == 1 && Some(one - 1) != skip {
                    let index = ones.binary_search(&(one - 1)).unwrap();
                    uf.union(i, index);
                }
                if one / n > 0 && grid[one / n - 1][one % n] == 1 && Some(one - n) != skip {
                    let index = ones.binary_search(&(one - n)).unwrap();
                    uf.union(i, index);
                }
            }
            if skip.is_some() {
                return uf.count > 2;
            }
            uf.count > 1
        }
        if is_disconnected(&grid, &ones, None, n) {
            return 0;
        }
        for i in 0..ones.len() {
            if is_disconnected(&grid, &ones, Some(ones[i]), n) {
                return 1;
            }
        }
        2
    }
}

struct UnionFind {
    parent: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        for (i, p) in parent.iter_mut().enumerate() {
            *p = i;
        }
        UnionFind { parent, count: n }
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
            self.count -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::ArrayStringExt;

    use super::*;

    #[test]
    fn test_min_days() {
        let grid = "[[0,1,1,0],[0,1,1,0],[0,0,0,0]]".to_matrix();
        let result = 2;
        assert_eq!(Solution::min_days_island(grid), result);
    }

    #[test]
    fn test_min_days_2() {
        let grid = "[[1,1]]".to_matrix();
        let result = 2;
        assert_eq!(Solution::min_days_island(grid), result);
    }

    #[test]
    fn test_min_days_3() {
        let grid = "[[1,0,1,0]]".to_matrix();
        let result = 0;
        assert_eq!(Solution::min_days_island(grid), result);
    }

    #[test]
    fn test_min_days_4() {
        let grid = "[[0,0,0],[0,1,0],[0,0,0]]".to_matrix();
        let result = 1;
        assert_eq!(Solution::min_days_island(grid), result);
    }
}
