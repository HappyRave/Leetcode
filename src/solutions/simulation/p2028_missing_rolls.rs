use crate::solutions::Solution;

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let (m, sum) = (rolls.len() as i32, rolls.iter().sum::<i32>());
        let missing = (m + n) * mean - sum;
        if missing < n || missing > 6 * n {
            return vec![];
        }
        let mut res = vec![missing / n; n as usize];
        for i in 0..(missing % n) {
            res[i as usize] += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_rolls() {
        let rolls = vec![3, 2, 4, 3];
        let mean = 4;
        let n = 2;
        let res = vec![6, 6];
        assert_eq!(Solution::missing_rolls(rolls, mean, n), res);
    }

    #[test]
    fn test_missing_rolls_2() {
        let rolls = vec![1, 5, 6];
        let mean = 3;
        let n = 4;
        let mut res = vec![2, 3, 2, 2];
        res.sort_unstable_by(|a, b| b.cmp(a));
        assert_eq!(Solution::missing_rolls(rolls, mean, n), res);
    }

    #[test]
    fn test_missing_rolls_3() {
        let rolls = vec![1, 2, 3, 4];
        let mean = 6;
        let n = 4;
        let res = vec![];
        assert_eq!(Solution::missing_rolls(rolls, mean, n), res);
    }
}
