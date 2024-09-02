use crate::solutions::Solution;

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let prefix_sum = chalk
            .iter()
            .scan(0, |acc, &x| {
                *acc += x as i64;
                Some(*acc)
            })
            .collect::<Vec<i64>>();
        let k = k as i64 % prefix_sum.last().unwrap();
        let mut left = 0;
        let mut right = chalk.len() - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if prefix_sum[mid] <= k {
                left = mid + 1;
            } else {
                if mid == 0 || prefix_sum[0] > k {
                    return 0;
                }
                right = mid - 1;
            }
        }
        left as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chalk_replacer() {
        let chalk = vec![5, 1, 5];
        let k = 22;
        let res = 0;
        assert_eq!(Solution::chalk_replacer(chalk, k), res);
    }

    #[test]
    fn test_chalk_replacer_2() {
        let chalk = vec![3, 4, 1, 2];
        let k = 25;
        let res = 1;
        assert_eq!(Solution::chalk_replacer(chalk, k), res);
    }
}
