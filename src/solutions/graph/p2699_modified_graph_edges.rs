use crate::solutions::Solution;

impl Solution {
    pub fn modified_graph_edges(
        n: i32,
        mut edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
        target: i32,
    ) -> Vec<Vec<i32>> {
        use std::cmp::Reverse;
        fn run_dijkstra(
            graph: &[Vec<(usize, usize)>],
            edges: &mut [Vec<i32>],
            distances: &mut [[i32; 2]],
            source: usize,
            difference: i32,
            run: usize,
        ) {
            let mut heap = std::collections::BinaryHeap::new();
            heap.push(Reverse((0, source)));
            distances[source][run] = 0;

            while let Some(Reverse((current_distance, current_node))) = heap.pop() {
                if current_distance > distances[current_node][run] {
                    continue;
                }

                for &(next_node, edge_index) in &graph[current_node] {
                    let mut weight = edges[edge_index][2];
                    if weight == -1 {
                        weight = 1;
                    }

                    if run == 1 && edges[edge_index][2] == -1 {
                        let new_weight =
                            difference + distances[next_node][0] - distances[current_node][1];
                        if new_weight > weight {
                            edges[edge_index][2] = new_weight;
                            weight = new_weight;
                        }
                    }

                    let new_distance = distances[current_node][run].saturating_add(weight);
                    if new_distance < distances[next_node][run] {
                        distances[next_node][run] = new_distance;
                        heap.push(Reverse((new_distance, next_node)));
                    }
                }
            }
        }

        let mut graph = vec![vec![]; n as usize];
        for (i, edge) in edges.iter().enumerate() {
            graph[edge[0] as usize].push((edge[1] as usize, i));
            graph[edge[1] as usize].push((edge[0] as usize, i));
        }

        let mut distances = vec![[std::i32::MAX; 2]; n as usize];
        run_dijkstra(&graph, &mut edges, &mut distances, source as usize, 0, 0);
        let difference = target - distances[destination as usize][0];
        if difference < 0 {
            return vec![];
        }

        run_dijkstra(
            &graph,
            &mut edges,
            &mut distances,
            source as usize,
            difference,
            1,
        );
        if distances[destination as usize][1] < target {
            return vec![];
        }

        for edge in &mut edges {
            if edge[2] == -1 {
                edge[2] = 1;
            }
        }

        edges
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::ArrayStringExt;

    use super::*;

    #[test]
    fn test_modified_graph_edges() {
        let n = 5;
        let edges = "[[4,1,-1],[2,0,-1],[0,3,-1],[4,3,-1]]".to_matrix();
        let source = 0;
        let destination = 1;
        let target = 5;
        let result = "[[4,1,1],[2,0,3],[0,3,3],[4,3,1]]".to_matrix();
        assert_eq!(
            Solution::modified_graph_edges(n, edges, source, destination, target),
            result
        );
    }

    #[test]
    fn test_modified_graph_edges_2() {
        let n = 5;
        let edges = "[[0,1,-1],[0,2,5]]".to_matrix();
        let source = 0;
        let destination = 2;
        let target = 6;
        let result = "[]".to_matrix();
        assert_eq!(
            Solution::modified_graph_edges(n, edges, source, destination, target),
            result
        );
    }

    #[test]
    fn test_modified_graph_edges_3() {
        let n = 5;
        let edges = "[[1,0,4],[1,2,3],[2,3,5],[0,3,-1]]".to_matrix();
        let source = 0;
        let destination = 2;
        let target = 6;
        let result = "[[1,0,4],[1,2,3],[2,3,5],[0,3,1]]".to_matrix();
        assert_eq!(
            Solution::modified_graph_edges(n, edges, source, destination, target),
            result
        );
    }
}
