use crate::solutions::Solution;

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut count = 0;
        for k in (2..nums.len()).rev() {
            let mut i = 0;
            let mut j = k - 1;
            while i < j {
                if nums[i] + nums[j] > nums[k] {
                    count += j - i;
                    j -= 1;
                } else {
                    i += 1;
                }
            }
        }
        count as i32
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
    fn test_triangle_number_5() {
        assert_eq!(Solution::triangle_number(vec![0]), 0);
    }

    #[test]
    fn test_triangle_number_6() {
        assert_eq!(
            Solution::triangle_number(vec![24, 3, 82, 22, 35, 84, 19]),
            10
        );
    }
}
