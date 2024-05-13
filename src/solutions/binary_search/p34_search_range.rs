use crate::solutions::Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        let mut result = vec![-1, -1];
        while left <= right {
            let mid = left + (right - left) / 2;
            match nums[mid as usize].cmp(&target) {
                std::cmp::Ordering::Equal => {
                    let mut l = mid;
                    let mut r = mid;
                    while l >= 0 && nums[l as usize] == target {
                        l -= 1;
                    }
                    while r < nums.len() as i32 && nums[r as usize] == target {
                        r += 1;
                    }
                    result[0] = l + 1;
                    result[1] = r - 1;
                    break;
                }
                std::cmp::Ordering::Less => left = mid + 1,
                std::cmp::Ordering::Greater => right = mid - 1,
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_range() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;
        let result = Solution::search_range(nums, target);
        assert_eq!(result, vec![3, 4]);

        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 6;
        let result = Solution::search_range(nums, target);
        assert_eq!(result, vec![-1, -1]);

        let nums = vec![];
        let target = 0;
        let result = Solution::search_range(nums, target);
        assert_eq!(result, vec![-1, -1]);
    }
}
