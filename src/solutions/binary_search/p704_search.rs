use crate::solutions::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            match nums[mid as usize].cmp(&target) {
                std::cmp::Ordering::Equal => return mid,
                std::cmp::Ordering::Less => left = mid + 1,
                std::cmp::Ordering::Greater => right = mid - 1,
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        let result = Solution::search(nums, target);
        assert_eq!(result, 4);

        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        let result = Solution::search(nums, target);
        assert_eq!(result, -1);
    }
}
