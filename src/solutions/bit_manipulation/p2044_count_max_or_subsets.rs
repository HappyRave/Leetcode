use crate::solutions::Solution;

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut max_or = 0;
        for num in &nums {
            max_or |= num;
        }
        let mut count = 0;
        for i in 0..(1 << nums.len()) {
            let mut or = 0;
            for (j, num) in nums.iter().enumerate() {
                if i & (1 << j) != 0 {
                    or |= num;
                }
            }
            if or == max_or {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_max_or_subsets() {
        let nums = vec![3, 1];
        assert_eq!(Solution::count_max_or_subsets(nums), 2);
    }

    #[test]
    fn test_count_max_or_subsets_2() {
        let nums = vec![2, 2, 2];
        assert_eq!(Solution::count_max_or_subsets(nums), 7);
    }

    #[test]
    fn test_count_max_or_subsets_3() {
        let nums = vec![3, 2, 1, 5];
        assert_eq!(Solution::count_max_or_subsets(nums), 6);
    }
}
