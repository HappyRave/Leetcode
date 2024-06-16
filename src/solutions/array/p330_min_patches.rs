use crate::solutions::Solution;

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut patches = 0;
        let mut x: i64 = 1;
        let mut i = 0;
        let n = n as i64;
        let len = nums.len() as i64;
        while x <= n {
            if i < len && nums[i as usize] as i64 <= x {
                x += nums[i as usize] as i64;
                i += 1;
            } else {
                x *= 2;
                patches += 1;
            }
        }
        patches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_patches() {
        assert_eq!(Solution::min_patches(vec![1, 3], 6), 1);
    }

    #[test]
    fn test_min_patches_2() {
        assert_eq!(Solution::min_patches(vec![1, 5, 10], 20), 2);
    }

    #[test]
    fn test_min_patches_3() {
        assert_eq!(Solution::min_patches(vec![1, 2, 2], 5), 0);
    }

    #[test]
    fn test_min_patches_4() {
        assert_eq!(Solution::min_patches(vec![1, 2, 31, 33], 2147483647), 28);
    }
}
