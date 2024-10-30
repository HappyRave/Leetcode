use crate::solutions::Solution;

impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut longest_inc_subseq = vec![1; n];
        let mut longest_dec_subseq = vec![1; n];
        for i in 0..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    longest_inc_subseq[i] = longest_inc_subseq[i].max(longest_inc_subseq[j] + 1);
                }
            }
        }
        for i in (0..n).rev() {
            for j in (i + 1..n).rev() {
                if nums[i] > nums[j] {
                    longest_dec_subseq[i] = longest_dec_subseq[i].max(longest_dec_subseq[j] + 1);
                }
            }
        }

        let mut res = 0;
        for i in 0..n {
            if longest_inc_subseq[i] > 1 && longest_dec_subseq[i] > 1 {
                res = res.max(longest_inc_subseq[i] + longest_dec_subseq[i] - 1);
            }
        }
        (n - res) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_mountain_removals() {
        assert_eq!(Solution::minimum_mountain_removals(vec![1, 3, 1]), 0);
    }

    #[test]
    fn test_minimum_mountain_removals_2() {
        assert_eq!(
            Solution::minimum_mountain_removals(vec![2, 1, 1, 5, 6, 2, 3, 1]),
            3
        );
    }

    #[test]
    fn test_minimum_mountain_removals_3() {
        assert_eq!(
            Solution::minimum_mountain_removals(vec![4, 3, 2, 1, 1, 2, 3, 1]),
            4
        );
    }
}
