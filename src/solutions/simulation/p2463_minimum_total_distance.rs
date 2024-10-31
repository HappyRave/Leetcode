use crate::solutions::Solution;

impl Solution {
    pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
        robot.sort_unstable();
        factory.sort_unstable();

        let (m, n) = (robot.len(), factory.len());
        let mut dp = vec![vec![0i64; n + 1]; m + 1];

        for dp_row in dp.iter_mut() {
            dp_row[n] = i64::MAX;
        }

        for j in (0..n).rev() {
            let mut prefix = 0i64;
            let mut queue = std::collections::VecDeque::new();
            queue.push_back((m, 0i64));

            for i in (0..m).rev() {
                prefix += (robot[i] - factory[j][0]).abs() as i64;

                while !queue.is_empty() && queue.front().unwrap().0 > i + factory[j][1] as usize {
                    queue.pop_front();
                }

                while !queue.is_empty() && queue.back().unwrap().1 >= dp[i][j + 1] - prefix {
                    queue.pop_back();
                }

                queue.push_back((i, dp[i][j + 1] - prefix));
                dp[i][j] = prefix + queue.front().unwrap().1;
            }
        }

        dp[0][0]
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::ArrayStringExt;

    use super::*;

    #[test]
    fn test_minimum_total_distance() {
        let robot = vec![0, 4, 6];
        let factory = "[[2,2],[6,2]]".to_matrix();
        assert_eq!(Solution::minimum_total_distance(robot, factory), 4);
    }

    #[test]
    fn test_minimum_total_distance_2() {
        let robot = vec![1, -1];
        let factory = "[[-2,1],[2,1]]".to_matrix();
        assert_eq!(Solution::minimum_total_distance(robot, factory), 2);
    }
}
