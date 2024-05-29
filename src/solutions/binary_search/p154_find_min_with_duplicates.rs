use crate::solutions::Solution;

impl Solution {
    pub fn find_min_duplicates(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left < right {
            let mid = left + (right - left) / 2;
            match nums[mid as usize].cmp(&nums[right as usize]) {
                std::cmp::Ordering::Less => right = mid,
                std::cmp::Ordering::Greater => left = mid + 1,
                std::cmp::Ordering::Equal => right -= 1,
            }
        }
        nums[left as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_duplicates() {
        assert_eq!(Solution::find_min_duplicates(vec![1, 3, 5]), 1);
    }

    #[test]
    fn test_find_min_duplicates_2() {
        assert_eq!(Solution::find_min_duplicates(vec![2, 2, 2, 0, 1]), 0);
    }

    #[test]
    fn test_find_min_duplicates_3() {
        assert_eq!(Solution::find_min_duplicates(vec![11, 13, 15, 17]), 11);
    }
}
