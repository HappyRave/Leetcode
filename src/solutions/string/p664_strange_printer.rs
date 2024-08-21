use crate::solutions::Solution;

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let s = s.chars().fold(vec![], |mut acc, c| {
            if acc.last() != Some(&c) {
                acc.push(c);
            }
            acc
        });
        let n = s.len();
        let mut dp = vec![vec![0; n]; n];
        for i in (0..n).rev() {
            dp[i][i] = 1;
            for j in i + 1..n {
                dp[i][j] = dp[i + 1][j] + 1;
                for k in i + 1..=j {
                    if s[i] == s[k] {
                        dp[i][j] = dp[i][j].min(dp[i + 1][k - 1] + dp[k][j]);
                    }
                }
            }
        }
        dp[0][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strange_printer() {
        assert_eq!(Solution::strange_printer("aaabbb".to_string()), 2); // cspell: disable-line
    }

    #[test]
    fn test_strange_printer_2() {
        assert_eq!(Solution::strange_printer("aba".to_string()), 2);
    }
}
