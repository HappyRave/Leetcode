use crate::solutions::Solution;

impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        //nums.sort_unstable();
        let n = nums.len() as i32;
        for i in 1..=n {
            if nums.iter().filter(|&&x| x >= i).count() == i as usize {
                return i;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_special_array() {
        let nums = vec![3, 5];
        let res = 2;
        assert_eq!(Solution::special_array(nums), res);
    }

    #[test]
    fn test_special_array_2() {
        let nums = vec![0, 0];
        let res = -1;
        assert_eq!(Solution::special_array(nums), res);
    }

    #[test]
    fn test_special_array_3() {
        let nums = vec![0, 4, 3, 0, 4];
        let res = 3;
        assert_eq!(Solution::special_array(nums), res);
    }

    #[test]
    fn test_special_array_4() {
        let nums = vec![3, 6, 7, 7, 0];
        let res = -1;
        assert_eq!(Solution::special_array(nums), res);
    }
}
