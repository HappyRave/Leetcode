use crate::solutions::Solution;

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let stone_count = stones.len();
        let mut uf = UnionFind::new(stone_count);
        for i in 0..stone_count {
            for j in i + 1..stone_count {
                if stones[i][0] == stones[j][0] || stones[i][1] == stones[j][1] {
                    uf.union(i as i32, j as i32);
                }
            }
        }
        stone_count as i32 - uf.size as i32
    }
}

struct UnionFind {
    parent: Vec<i32>,
    size: usize,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size as i32).collect(),
            size,
        }
    }

    fn find(&mut self, mut x: i32) -> i32 {
        while self.parent[x as usize] != x {
            self.parent[x as usize] = self.parent[self.parent[x as usize] as usize];
            x = self.parent[x as usize];
        }
        x
    }

    fn union(&mut self, x: i32, y: i32) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            self.parent[root_x as usize] = root_y;
            self.size -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::ArrayStringExt;

    use super::*;

    #[test]
    fn test_remove_stones() {
        let stones = "[[0,0],[0,1],[1,0],[1,2],[2,1],[2,2]]".to_matrix();
        assert_eq!(Solution::remove_stones(stones), 5);
    }

    #[test]
    fn test_remove_stones_2() {
        let stones = "[[0,0],[0,2],[1,1],[2,0],[2,2]]".to_matrix();
        assert_eq!(Solution::remove_stones(stones), 3);
    }
}
