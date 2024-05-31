use crate::solutions::Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xor = nums.iter().fold(0, |acc, &x| acc ^ x);
        let mask = xor & (-xor);
        let mut res = vec![0, 0];
        nums.iter().for_each(|&x| {
            if x & mask == 0 {
                res[0] ^= x;
            } else {
                res[1] ^= x;
            }
        });
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 1, 3, 2, 5];
        let res = vec![3, 5];
        let mut sol = Solution::single_number(nums);
        sol.sort_unstable();
        assert_eq!(sol, res);
    }

    #[test]
    fn test_2() {
        let nums = vec![-1, 0];
        let res = vec![-1, 0];
        let mut sol = Solution::single_number(nums);
        sol.sort_unstable();
        assert_eq!(sol, res);
    }

    #[test]
    fn test_3() {
        let nums = vec![0, 1];
        let res = vec![0, 1];
        let mut sol = Solution::single_number(nums);
        sol.sort_unstable();
        assert_eq!(sol, res);
    }
}
