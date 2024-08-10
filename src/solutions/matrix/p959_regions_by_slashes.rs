use crate::solutions::Solution;

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let n = grid.len();
        let mut uf = UnionFind::new(n * n * 4);
        for (i, row) in grid.iter().enumerate().map(|(i, row)| (i, row.as_bytes())) {
            for (j, cell) in row.iter().enumerate() {
                let index = i * n + j;
                let start = index * 4;
                match cell {
                    b'/' => {
                        uf.union(start, start + 3);
                        uf.union(start + 1, start + 2);
                    }
                    b'\\' => {
                        uf.union(start, start + 1);
                        uf.union(start + 2, start + 3);
                    }
                    _ => {
                        uf.union(start, start + 1);
                        uf.union(start + 1, start + 2);
                        uf.union(start + 2, start + 3);
                    }
                }
                if i > 0 {
                    uf.union(start, start - 4 * n + 2);
                }
                if j > 0 {
                    uf.union(start + 3, start - 3);
                }
            }
        }
        uf.count as i32
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
    use super::*;

    #[test]
    fn test_regions_by_slashes() {
        assert_eq!(
            Solution::regions_by_slashes(vec![" /".to_string(), "/ ".to_string()]),
            2
        );
    }

    #[test]
    fn test_regions_by_slashes_2() {
        assert_eq!(
            Solution::regions_by_slashes(vec![" /".to_string(), "  ".to_string()]),
            1
        );
    }

    #[test]
    fn test_regions_by_slashes_3() {
        assert_eq!(
            Solution::regions_by_slashes(vec!["\\/".to_string(), "/\\".to_string()]),
            4
        );
    }

    #[test]
    fn test_regions_by_slashes_4() {
        assert_eq!(
            Solution::regions_by_slashes(vec![
                "/\\ ".to_string(),
                "\\/ ".to_string(),
                " / ".to_string()
            ]),
            3
        );
    }
}
