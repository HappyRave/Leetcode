use crate::solutions::Solution;

impl Solution {
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let freq = nums.iter().fold(
            std::collections::HashMap::new(),
            |mut acc: std::collections::HashMap<i32, usize>, &x| {
                *acc.entry(x).or_default() += 1;
                acc
            },
        );

        nums.sort_by(|a, b| {
            freq.get(a)
                .unwrap()
                .cmp(freq.get(b).unwrap())
                .then(b.cmp(a))
        });
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_sort() {
        let nums = vec![1, 1, 2, 2, 2, 3];
        let result = vec![3, 1, 1, 2, 2, 2];
        assert_eq!(Solution::frequency_sort(nums), result);
    }

    #[test]
    fn test_frequency_sort_2() {
        let nums = vec![2, 3, 1, 3, 2];
        let result = vec![1, 3, 3, 2, 2];
        assert_eq!(Solution::frequency_sort(nums), result);
    }

    #[test]
    fn test_frequency_sort_3() {
        let nums = vec![-1, 1, -6, 4, 5, -6, 1, 4, 1];
        let result = vec![5, -1, 4, 4, -6, -6, 1, 1, 1];
        assert_eq!(Solution::frequency_sort(nums), result);
    }
}
