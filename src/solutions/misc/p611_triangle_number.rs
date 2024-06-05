use crate::solutions::Solution;

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        }
        nums.sort();
        let mut count = 0;
        (0..nums.len() - 2).for_each(|i| {
            (i + 1..nums.len() - 1).for_each(|j| {
                let side = nums[i] + nums[j];
                let shorter = nums[j + 1..].partition_point(|&x| x < side);
                count += shorter as i32;
            });
        });
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_number() {
        assert_eq!(Solution::triangle_number(vec![2, 2, 3, 4]), 3);
    }

    #[test]
    fn test_triangle_number_2() {
        assert_eq!(Solution::triangle_number(vec![4, 2, 3, 4]), 4);
    }

    #[test]
    fn test_triangle_number_3() {
        assert_eq!(Solution::triangle_number(vec![0, 1, 0]), 0);
    }

    #[test]
    fn test_triangle_number_4() {
        assert_eq!(Solution::triangle_number(vec![0, 0, 0]), 0);
    }

    #[test]
    fn test_triangle_number_() {
        assert_eq!(Solution::triangle_number(vec![0]), 0);
    }
}
