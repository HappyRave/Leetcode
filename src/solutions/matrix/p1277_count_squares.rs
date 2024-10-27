use crate::solutions::Solution;

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut dp = vec![vec![0; matrix[0].len()]; matrix.len()];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 1 {
                    dp[i][j] = 1;
                    if i > 0 && j > 0 {
                        dp[i][j] += std::cmp::min(
                            dp[i - 1][j],
                            std::cmp::min(dp[i][j - 1], dp[i - 1][j - 1]),
                        );
                    }
                    result += dp[i][j];
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
    fn test_count_squares() {
        let matrix = vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]];
        assert_eq!(Solution::count_squares(matrix), 15);
    }

    #[test]
    fn test_count_squares_2() {
        let matrix = vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(Solution::count_squares(matrix), 7);
    }
}
