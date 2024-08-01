use crate::solutions::Solution;

impl Solution {
    pub fn distinct_sequences(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        fn is_gcd_one(a: i32, b: i32) -> bool {
            match (a, b) {
                (3, 1) | (1, 3) | (5, 1) | (1, 5) | (5, 2) | (2, 5) | (5, 3) | (3, 5) => false,
                (x, y) => x != y,
            }
        }

        let mut dp = vec![vec![vec![0; 7]; 7]; n as usize + 1];

        fn dfs(n: i32, prev: i32, prev_prev: i32, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
            if n == 0 {
                return 1;
            }
            if dp[n as usize][prev as usize][prev_prev as usize] == 0 {
                for roll in 0..6 {
                    if roll != prev && roll != prev_prev && is_gcd_one(prev, roll) {
                        dp[n as usize][prev as usize][prev_prev as usize] +=
                            dfs(n - 1, roll, prev, dp);
                        dp[n as usize][prev as usize][prev_prev as usize] %= 1_000_000_007;
                    }
                }
            }
            dp[n as usize][prev as usize][prev_prev as usize]
        }

        dfs(n, 6, 6, &mut dp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distinct_sequences() {
        assert_eq!(Solution::distinct_sequences(4), 184);
    }

    #[test]
    fn test_distinct_sequences_2() {
        assert_eq!(Solution::distinct_sequences(2), 22);
    }

    #[test]
    fn test_distinct_sequences_3() {
        assert_eq!(Solution::distinct_sequences(8), 11672);
    }
}
