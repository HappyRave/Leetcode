use crate::solutions::Solution;

impl Solution {
    pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        let n = n as usize;
        let roll_max: Vec<usize> = roll_max.iter().map(|&x| x as usize).collect();
        let mut dp = vec![[1i64; 6]; n];

        for roll in 1..n {
            for (outcome, max_roll) in roll_max.iter().enumerate() {
                dp[roll][outcome] = 0;
                for prev_outcome in 0..6 {
                    dp[roll][outcome] += dp[roll - 1][prev_outcome];
                }

                match max_roll.cmp(&roll) {
                    std::cmp::Ordering::Less => {
                        for prev_roll in 0..6 {
                            if prev_roll != outcome {
                                dp[roll][outcome] -= dp[roll - *max_roll - 1][prev_roll];
                            }
                        }
                    }
                    std::cmp::Ordering::Equal => {
                        dp[roll][outcome] -= 1;
                    }
                    std::cmp::Ordering::Greater => {}
                }

                dp[roll][outcome] %= 1_000_000_007;
                if dp[roll][outcome] < 0 {
                    dp[roll][outcome] += 1_000_000_007;
                }
            }
        }

        (dp[n - 1].iter().sum::<i64>() % 1_000_000_007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_die_simulator() {
        assert_eq!(Solution::die_simulator(2, vec![1, 1, 2, 2, 2, 3]), 34);
    }

    #[test]
    fn test_die_simulator_2() {
        assert_eq!(Solution::die_simulator(2, vec![1, 1, 1, 1, 1, 1]), 30);
    }

    #[test]
    fn test_die_simulator_3() {
        assert_eq!(Solution::die_simulator(3, vec![1, 1, 1, 2, 2, 3]), 181);
    }

    #[test]
    fn test_die_simulator_4() {
        assert_eq!(Solution::die_simulator(4, vec![2, 1, 1, 3, 3, 2]), 1082);
    }

    #[test]
    fn test_die_simulator_5() {
        assert_eq!(
            Solution::die_simulator(100, vec![8, 11, 13, 9, 10, 7]),
            778061023
        );
    }

    #[test]
    fn test_die_simulator_6() {
        assert_eq!(
            Solution::die_simulator(20, vec![8, 5, 10, 8, 7, 2]),
            822005673
        );
    }

    #[test]
    fn test_die_simulator_7() {
        assert_eq!(Solution::die_simulator(5, vec![3, 1, 1, 1, 1, 1]), 4330);
    }
}
