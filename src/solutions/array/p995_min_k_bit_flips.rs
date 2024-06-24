use crate::solutions::Solution;

impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut flips = 0;
        let mut flip = vec![0; nums.len()];
        let mut count = 0;
        for i in 0..nums.len() {
            if i >= k {
                count ^= flip[i - k];
            }
            if count % 2 == nums[i] {
                if i + k > nums.len() {
                    return -1;
                }
                flip[i] = 1;
                count ^= 1;
                flips += 1;
            }
        }
        flips
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_k_bit_flips() {
        assert_eq!(Solution::min_k_bit_flips(vec![], 0), 0);
    }

    #[test]
    fn test_min_k_bit_flips_2() {
        assert_eq!(Solution::min_k_bit_flips(vec![0, 1, 0], 1), 2);
    }

    #[test]
    fn test_min_k_bit_flips_3() {
        assert_eq!(Solution::min_k_bit_flips(vec![1, 1, 0], 2), -1);
    }

    #[test]
    fn test_min_k_bit_flips_4() {
        assert_eq!(
            Solution::min_k_bit_flips(vec![0, 0, 0, 1, 0, 1, 1, 0], 3),
            3
        );
    }
}
