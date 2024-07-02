use crate::solutions::Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for num in nums1 {
            *map.entry(num).or_insert(0) += 1;
        }
        let mut result = Vec::new();
        for num in nums2 {
            if let Some(count) = map.get_mut(&num) {
                if *count > 0 {
                    result.push(num);
                    *count -= 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect() {
        assert_eq!(
            Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2, 2]
        );
    }

    #[test]
    fn test_intersect_2() {
        let mut result = Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        result.sort_unstable();
        assert_eq!(result, vec![4, 9]);
    }
}
