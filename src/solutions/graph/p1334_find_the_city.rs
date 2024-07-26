use crate::solutions::Solution;

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let mut dist = vec![vec![std::i32::MAX; n]; n];
        (0..n).for_each(|i| {
            dist[i][i] = 0;
        });
        for edge in edges {
            let (u, v, w) = (edge[0] as usize, edge[1] as usize, edge[2]);
            dist[u][v] = w;
            dist[v][u] = w;
        }
        for k in 0..n {
            for i in 0..n {
                if dist[i][k] == std::i32::MAX {
                    continue;
                }
                for j in 0..n {
                    if dist[k][j] == std::i32::MAX {
                        continue;
                    }
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
        dist.iter()
            .map(|d| d.iter().filter(|&&x| x <= distance_threshold).count() - 1)
            .enumerate()
            .min_by_key(|&(city, count)| (count, -(city as i32)))
            .unwrap()
            .0 as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::ArrayStringExt;

    use super::*;

    #[test]
    fn p1334_find_the_city() {
        let edges = "[[0,1,3],[1,2,1],[1,3,4],[2,3,1]]".to_matrix();
        assert_eq!(Solution::find_the_city(4, edges, 4), 3);
    }

    #[test]
    fn p1334_find_the_city_2() {
        let edges = "[[0,1,2],[0,4,8],[1,2,3],[1,4,2],[2,3,1],[3,4,1]]".to_matrix();
        assert_eq!(Solution::find_the_city(5, edges, 2), 0);
    }
}
