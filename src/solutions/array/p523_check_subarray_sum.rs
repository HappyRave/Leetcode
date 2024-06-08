use crate::solutions::Solution;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut sum = 0;
        let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        map.insert(0, -1);
        #[allow(clippy::needless_range_loop)]
        for i in 0..nums.len() {
            sum += nums[i];
            if k != 0 {
                sum %= k;
            }
            if let Some(&prev) = map.get(&sum) {
                if i as i32 - prev > 1 {
                    return true;
                }
            } else {
                map.insert(sum, i as i32);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_subarray_sum() {
        let nums = vec![23, 2, 4, 6, 7];
        let k = 6;
        assert!(Solution::check_subarray_sum(nums, k));
    }

    #[test]
    fn test_check_subarray_sum_2() {
        let nums = vec![23, 2, 6, 4, 7];
        let k = 13;
        assert!(!Solution::check_subarray_sum(nums, k));
    }

    #[test]
    fn test_check_subarray_sum_3() {
        let nums = vec![2, 4, 3];
        let k = 6;
        assert!(Solution::check_subarray_sum(nums, k));
    }
}
