use crate::solutions::Solution;

impl Solution {
    pub fn count_operations_to_empty_array(mut nums: Vec<i32>) -> i64 {
        let mut map = std::collections::HashMap::new();
        let n = nums.len() as i64;
        let mut solution = n;
        for (i, &num) in nums.iter().enumerate() {
            map.insert(num, i);
        }
        nums.sort_unstable();
        for i in 1..nums.len() {
            if map[&nums[i]] < map[&nums[i - 1]] {
                solution += n - i as i64;
            }
        }

        solution
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![3, 4, -1];
        assert_eq!(Solution::count_operations_to_empty_array(nums), 5);

        let nums = vec![1, 2, 4, 3];
        assert_eq!(Solution::count_operations_to_empty_array(nums), 5);

        let nums = vec![1, 2, 3];
        assert_eq!(Solution::count_operations_to_empty_array(nums), 3);
    }
}
