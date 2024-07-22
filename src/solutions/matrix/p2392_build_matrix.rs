use crate::solutions::Solution;

impl Solution {
    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        fn topology(k: i32, conditions: Vec<Vec<i32>>) -> Vec<i32> {
            let mut adjacency_list = vec![vec![]; k as usize + 1];
            let mut in_degree = vec![0; k as usize + 1];
            conditions.iter().for_each(|condition| {
                let (from, to) = (condition[0] as usize, condition[1] as usize);
                adjacency_list[from].push(to);
                in_degree[to] += 1;
            });

            let mut queue: std::collections::VecDeque<usize> = in_degree
                .iter()
                .enumerate()
                .filter(|&(index, &degree)| (index != 0 && degree == 0))
                .map(|(index, _)| index)
                .collect();

            let mut sorted_order = Vec::with_capacity(k as usize);
            while let Some(node) = queue.pop_front() {
                sorted_order.push(node as i32);
                for &adjacent in &adjacency_list[node] {
                    in_degree[adjacent] -= 1;
                    if in_degree[adjacent] == 0 {
                        queue.push_back(adjacent);
                    }
                }
            }

            if sorted_order.len() == k as usize {
                sorted_order
            } else {
                vec![]
            }
        }

        fn build_matrix(k: i32, row_topo: Vec<i32>, col_topo: Vec<i32>) -> Vec<Vec<i32>> {
            let mut rows_map = std::collections::HashMap::new();
            col_topo.iter().enumerate().for_each(|(i, &c)| {
                let mut row = vec![0; k as usize];
                row[i] = c;
                rows_map.insert(c, row);
            });
            row_topo
                .iter()
                .map(|&r| rows_map.get(&r).unwrap().clone())
                .collect()
        }

        let row_topo = topology(k, row_conditions);
        if row_topo.is_empty() {
            return vec![];
        }
        let col_topo = topology(k, col_conditions);
        if col_topo.is_empty() {
            return vec![];
        }
        build_matrix(k, row_topo, col_topo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_matrix() {
        let k = 3;
        let row_conditions = vec![vec![1, 2], vec![3, 2]];
        let col_conditions = vec![vec![2, 1], vec![3, 2]];
        let result = Solution::build_matrix(k, row_conditions, col_conditions);
        let expected = vec![vec![0, 0, 1], vec![3, 0, 0], vec![0, 2, 0]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_build_matrix_2() {
        let k = 3;
        let row_conditions = vec![vec![1, 2], vec![2, 3], vec![3, 1], vec![2, 3]];
        let col_conditions = vec![vec![2, 1]];
        let result = Solution::build_matrix(k, row_conditions, col_conditions);
        let expected: Vec<Vec<_>> = vec![];
        assert_eq!(result, expected);
    }
}
