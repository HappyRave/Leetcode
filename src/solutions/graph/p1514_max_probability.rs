use crate::solutions::Solution;

impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start_node: i32,
        end_node: i32,
    ) -> f64 {
        let mut graph = vec![vec![]; n as usize];
        for (edge, p) in edges.iter().zip(succ_prob) {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            graph[u].push((v, p));
            graph[v].push((u, p));
        }

        let mut prob = vec![0.0; n as usize];
        let mut heap = std::collections::BinaryHeap::new();
        heap.push((Ordf64(1f64), start_node as usize));
        let mut visited = std::collections::HashSet::new();

        while let Some((Ordf64(p), u)) = heap.pop() {
            if visited.contains(&u) {
                continue;
            }
            visited.insert(u);
            prob[u] = p;

            if u == end_node as usize {
                return p;
            }

            for &(v, p) in graph[u].iter() {
                if !visited.contains(&v) {
                    heap.push((Ordf64(p * prob[u]), v));
                }
            }
        }
        0f64
    }
}

#[derive(PartialEq, PartialOrd)]
struct Ordf64(f64);
impl Eq for Ordf64 {}
#[allow(clippy::derive_ord_xor_partial_ord)]
impl std::cmp::Ord for Ordf64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::ArrayStringExt;

    use super::*;

    #[test]
    fn test_max_probability() {
        let n = 3;
        let edges = "[[0,1],[1,2],[0,2]]".to_matrix();
        let succ_prob = vec![0.5, 0.5, 0.2];
        let start_node = 0;
        let end_node = 2;
        assert_eq!(
            Solution::max_probability(n, edges, succ_prob, start_node, end_node),
            0.25000
        );
    }

    #[test]
    fn test_max_probability_2() {
        let n = 3;
        let edges = "[[0,1],[1,2],[0,2]]".to_matrix();
        let succ_prob = vec![0.5, 0.5, 0.3];
        let start_node = 0;
        let end_node = 2;
        assert_eq!(
            Solution::max_probability(n, edges, succ_prob, start_node, end_node),
            0.30000
        );
    }

    #[test]
    fn test_max_probability_3() {
        let n = 3;
        let edges = "[[0,1]]".to_matrix();
        let succ_prob = vec![0.5];
        let start_node = 0;
        let end_node = 2;
        assert_eq!(
            Solution::max_probability(n, edges, succ_prob, start_node, end_node),
            0.00000
        );
    }
}
