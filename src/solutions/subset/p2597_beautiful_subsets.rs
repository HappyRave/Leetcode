use crate::solutions::Solution;

impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        fn backtrack(nums: &Vec<i32>, k: i32, start: usize, path: &mut Vec<i32>, result: &mut i32) {
            if !path.is_empty() {
                *result += 1;
            }
            (start..nums.len()).for_each(|i| {
                if path.iter().all(|&x| (x - nums[i]).abs() != k) {
                    path.push(nums[i]);
                    backtrack(nums, k, i + 1, path, result);
                    path.pop();
                }
            });
        }

        let mut result = 0;
        let mut path = Vec::new();
        backtrack(&nums, k, 0, &mut path, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beautiful_subsets() {
        assert_eq!(Solution::beautiful_subsets(vec![2, 4, 6], 2), 4);
    }

    #[test]
    fn test_beautiful_subsets_2() {
        assert_eq!(Solution::beautiful_subsets(vec![1], 1), 1);
    }
}
