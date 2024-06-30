use crate::solutions::Solution;

struct UnionFind {
    parent: Vec<usize>,
    set_count: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect::<Vec<_>>();
        UnionFind {
            parent,
            set_count: 1,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            self.parent[root_x] = root_y;
            self.set_count += 1;
            true
        } else {
            false
        }
    }
}

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, mut edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut alice = UnionFind::new(n);
        let mut bob = UnionFind::new(n);
        let mut result = 0;
        edges.sort_unstable_by_key(|edge| -edge[0]);
        for edge in edges.iter() {
            let edge_type = edge[0];
            let u = edge[1] as usize - 1;
            let v = edge[2] as usize - 1;
            if edge_type == 3 {
                let union_alice = alice.union(u, v);
                let union_bob = bob.union(u, v);
                if !union_alice && !union_bob {
                    result += 1;
                }
            } else if edge_type == 1 && !alice.union(u, v) || edge_type == 2 && !bob.union(u, v) {
                result += 1;
            }
        }
        if alice.set_count < n || bob.set_count < n {
            return -1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_num_edges_to_remove() {
        let n = 4;
        let edges = vec![
            vec![3, 1, 2],
            vec![3, 2, 3],
            vec![1, 1, 3],
            vec![1, 2, 4],
            vec![1, 1, 2],
            vec![2, 3, 4],
        ];
        let result = 2;
        assert_eq!(Solution::max_num_edges_to_remove(n, edges), result);
    }

    #[test]
    fn test_max_num_edges_to_remove_2() {
        let n = 4;
        let edges = vec![vec![3, 1, 2], vec![3, 2, 3], vec![1, 1, 4], vec![2, 1, 4]];
        let result = 0;
        assert_eq!(Solution::max_num_edges_to_remove(n, edges), result);
    }

    #[test]
    fn test_max_num_edges_to_remove_3() {
        let n = 4;
        let edges = vec![vec![3, 1, 2], vec![3, 2, 3], vec![1, 1, 2], vec![2, 2, 4]];
        let result = -1;
        assert_eq!(Solution::max_num_edges_to_remove(n, edges), result);
    }

    #[test]
    fn test_max_num_edges_to_remove_4() {
        let n = 2;
        let edges = vec![vec![1, 1, 2], vec![2, 1, 2], vec![3, 1, 2]];
        let result = 2;
        assert_eq!(Solution::max_num_edges_to_remove(n, edges), result);
    }
}
