use crate::solutions::Solution;

impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let mut cities = vec![0; n as usize];
        for road in &roads {
            cities[road[0] as usize] += 1;
            cities[road[1] as usize] += 1;
        }
        cities.sort_unstable();
        cities
            .into_iter()
            .enumerate()
            .map(|(i, c)| (i + 1) as i64 * c)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_importance() {
        assert_eq!(
            Solution::maximum_importance(
                5,
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![2, 3],
                    vec![0, 2],
                    vec![1, 3],
                    vec![2, 4],
                ]
            ),
            43
        );
    }

    #[test]
    fn test_maximum_importance_2() {
        assert_eq!(
            Solution::maximum_importance(5, vec![vec![0, 3], vec![2, 4], vec![1, 3]]),
            20
        );
    }
}
