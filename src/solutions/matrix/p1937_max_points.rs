use crate::solutions::Solution;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let n = points[0].len();
        let points = points
            .iter()
            .map(|row| row.iter().map(|&n| n as i64).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut dp = points[0].to_vec();
        let mut maxi: i64;
        for row in points.iter().skip(1) {
            let mut left_to_right = vec![0; n];
            maxi = 0;
            for j in 0..n {
                maxi = std::cmp::max(maxi - 1, dp[j]);
                left_to_right[j] = maxi;
            }
            maxi = 0;
            for j in (0..n).rev() {
                maxi = std::cmp::max(maxi - 1, dp[j]);
                dp[j] = std::cmp::max(maxi, left_to_right[j]) + row[j];
            }
        }
        *dp.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::ArrayStringExt;

    use super::*;

    #[test]
    fn test_max_points() {
        let points = "[[1,2,3],[1,5,1],[3,1,1]]".to_matrix();
        let result = 9;
        assert_eq!(Solution::max_points(points), result);
    }

    #[test]
    fn test_max_points_2() {
        let points = "[[1,5],[2,3],[4,2]]".to_matrix();
        let result = 11;
        assert_eq!(Solution::max_points(points), result);
    }
}
