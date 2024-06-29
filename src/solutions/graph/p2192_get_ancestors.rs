use crate::solutions::Solution;

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        let mut degree = vec![0; n];

        for edge in edges.iter() {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            degree[v] += 1;
        }

        let mut result = vec![vec![]; n];
        let mut queue = degree
            .iter()
            .enumerate()
            .filter_map(|(i, &d)| if d == 0 { Some(i) } else { None })
            .collect::<std::collections::VecDeque<usize>>();

        while let Some(u) = queue.pop_front() {
            result[u].sort();
            result[u].dedup();
            for &v in graph[u].iter() {
                let u_ancestors = result[u].clone();
                result[v].extend(u_ancestors.iter());
                result[v].push(u as i32);
                degree[v] -= 1;
                if degree[v] == 0 {
                    queue.push_back(v);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ancestors() {
        let n = 8;
        let edges = vec![
            vec![0, 3],
            vec![0, 4],
            vec![1, 3],
            vec![2, 4],
            vec![2, 7],
            vec![3, 5],
            vec![3, 6],
            vec![3, 7],
            vec![4, 6],
        ];
        let res = vec![
            vec![],
            vec![],
            vec![],
            vec![0, 1],
            vec![0, 2],
            vec![0, 1, 3],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 2, 3],
        ];

        assert_eq!(Solution::get_ancestors(n, edges), res);
    }

    #[test]
    fn test_get_ancestors_2() {
        let n = 5;
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![0, 4],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
        ];
        let res = vec![vec![], vec![0], vec![0, 1], vec![0, 1, 2], vec![0, 1, 2, 3]];

        assert_eq!(Solution::get_ancestors(n, edges), res);
    }
}
