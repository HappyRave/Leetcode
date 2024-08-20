use crate::solutions::Solution;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        let mut prefix_sum = vec![0; n + 1];
        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + piles[i];
        }
        let mut dp = vec![vec![0; n + 1]; n + 1];
        for i in (0..n).rev() {
            for m in 1..=n {
                for x in 1..=2 * m {
                    if i + x > n {
                        break;
                    }
                    dp[i][m] = dp[i][m].max(prefix_sum[n] - prefix_sum[i] - dp[i + x][m.max(x)]);
                }
            }
        }
        dp[0][1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stone_game_ii() {
        assert_eq!(Solution::stone_game_ii(vec![2, 7, 9, 4, 4]), 10);
    }

    #[test]
    fn test_stone_game_ii_2() {
        assert_eq!(Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 100]), 104);
    }
}
