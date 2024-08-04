use crate::solutions::Solution;

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut right = arr.len() as i32;
        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid as usize] - mid - 1 < k {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left + k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kth_positive() {
        let arr = vec![2, 3, 4, 7, 11];
        let k = 5;
        assert_eq!(Solution::find_kth_positive(arr, k), 9);
    }

    #[test]
    fn test_find_kth_positive_2() {
        let arr = vec![1, 2, 3, 4];
        let k = 2;
        assert_eq!(Solution::find_kth_positive(arr, k), 6);
    }
}
