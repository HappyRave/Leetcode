use crate::solutions::Solution;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let remainder = nums.iter().map(|&x| x as i64).sum::<i64>() % p as i64;

        if remainder == 0 {
            return 0;
        }

        let mut map = std::collections::HashMap::new();
        map.insert(0, -1);
        let mut min_len = nums.len() as i32;
        let mut sum = 0;
        for (i, &x) in nums.iter().enumerate() {
            sum += x as i64;
            let target = (sum - remainder) % p as i64;
            if let Some(&j) = map.get(&target) {
                min_len = min_len.min(i as i32 - j);
            }
            map.insert(sum % p as i64, i as i32);
        }

        if min_len == nums.len() as i32 {
            -1
        } else {
            min_len
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_subarray() {
        let nums = vec![3, 1, 4, 2];
        let p = 6;
        let result = 1;
        assert_eq!(Solution::min_subarray(nums, p), result);
    }

    #[test]
    fn test_min_subarray_2() {
        let nums = vec![6, 3, 5, 2];
        let p = 9;
        let result = 2;
        assert_eq!(Solution::min_subarray(nums, p), result);
    }

    #[test]
    fn test_min_subarray_3() {
        let nums = vec![1, 2, 3];
        let p = 3;
        let result = 0;
        assert_eq!(Solution::min_subarray(nums, p), result);
    }

    #[test]
    fn test_min_subarray_4() {
        let nums = vec![1, 2, 3];
        let p = 7;
        let result = -1;
        assert_eq!(Solution::min_subarray(nums, p), result);
    }

    #[test]
    fn test_min_subarray_5() {
        let nums = vec![1000000000, 1000000000, 1000000000];
        let p = 3;
        let result = 0;
        assert_eq!(Solution::min_subarray(nums, p), result);
    }
}
