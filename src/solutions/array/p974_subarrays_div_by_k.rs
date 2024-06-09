use crate::solutions::Solution;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = vec![0; k as usize];
        count[0] = 1;
        let mut sum = 0;
        let mut result = 0;
        for num in nums {
            sum += num;
            let mut modulo = sum % k;
            if modulo < 0 {
                modulo += k;
            }
            result += count[modulo as usize];
            count[modulo as usize] += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subarrays_div_by_k() {
        let nums = vec![4, 5, 0, -2, -3, 1];
        let k = 5;
        assert_eq!(7, Solution::subarrays_div_by_k(nums, k));
    }

    #[test]
    fn test_subarrays_div_by_k_2() {
        let nums = vec![5];
        let k = 9;
        assert_eq!(0, Solution::subarrays_div_by_k(nums, k));
    }
}
