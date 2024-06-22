use crate::solutions::Solution;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut odd_count = 0;
        let mut result = 0;
        let mut prefix_sum = vec![0; nums.len() + 1];
        prefix_sum[0] = 1;
        for num in nums {
            if num % 2 == 1 {
                odd_count += 1;
            }
            if odd_count >= k {
                result += prefix_sum[(odd_count - k) as usize];
            }
            prefix_sum[odd_count as usize] += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_subarrays() {
        assert_eq!(Solution::number_of_subarrays(vec![], 0), 0);
    }

    #[test]
    fn test_number_of_subarrays_2() {
        assert_eq!(Solution::number_of_subarrays(vec![1, 1, 2, 1, 1], 3), 2);
    }

    #[test]
    fn test_number_of_subarrays_3() {
        assert_eq!(Solution::number_of_subarrays(vec![2, 4, 6], 1), 0);
    }

    #[test]
    fn test_number_of_subarrays_4() {
        assert_eq!(
            Solution::number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2),
            16
        );
    }
}
