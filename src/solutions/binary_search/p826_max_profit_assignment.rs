use crate::solutions::Solution;

impl Solution {
    pub fn max_profit_assignment(
        difficulty: Vec<i32>,
        profit: Vec<i32>,
        mut worker: Vec<i32>,
    ) -> i32 {
        let mut d: Vec<(i32, i32)> = difficulty.into_iter().zip(profit).collect();
        d.sort_unstable();
        worker.sort_unstable();
        let mut result = 0;
        let (mut i, mut max) = (0, 0);
        for w in worker {
            while i < d.len() && d[i].0 <= w {
                max = max.max(d[i].1);
                i += 1;
            }
            result += max;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit_assignment() {
        assert_eq!(
            Solution::max_profit_assignment(
                vec![2, 4, 6, 8, 10],
                vec![10, 20, 30, 40, 50],
                vec![4, 5, 6, 7]
            ),
            100
        );
    }

    #[test]
    fn test_max_profit_assignment_2() {
        assert_eq!(
            Solution::max_profit_assignment(vec![85, 47, 57], vec![24, 66, 99], vec![40, 25, 25]),
            0
        );
    }

    #[test]
    fn test_max_profit_assignment_3() {
        assert_eq!(
            Solution::max_profit_assignment(vec![17, 37, 58], vec![4, 90, 96], vec![34, 73, 45]),
            190
        );
    }

    #[test]
    fn test_max_profit_assignment_4() {
        assert_eq!(
            Solution::max_profit_assignment(
                vec![1, 1, 1, 1, 1, 1],
                vec![100, 1, 1, 1, 1, 1],
                vec![1, 1, 1]
            ),
            300
        );
    }

    #[test]
    fn test_max_profit_assignment_5() {
        assert_eq!(
            Solution::max_profit_assignment(
                vec![68, 35, 52, 47, 86],
                vec![67, 17, 1, 81, 3],
                vec![92, 10, 85, 84, 82]
            ),
            324
        );
    }
}
