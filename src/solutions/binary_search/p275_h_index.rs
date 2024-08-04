use crate::solutions::Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len() as i32;
        let mut left = 0;
        let mut right = n;
        while left < right {
            let mid = left + (right - left) / 2;
            if citations[mid as usize] < n - mid {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        n - left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_index() {
        let citations = vec![0, 1, 3, 5, 6];
        assert_eq!(Solution::h_index(citations), 3);
    }

    #[test]
    fn test_h_index_2() {
        let citations = vec![1, 2, 100];
        assert_eq!(Solution::h_index(citations), 2);
    }

    #[test]
    fn test_h_index_3() {
        let citations = vec![0];
        assert_eq!(Solution::h_index(citations), 0);
    }

    #[test]
    fn test_h_index_4() {
        let citations = vec![100];
        assert_eq!(Solution::h_index(citations), 1);
    }

    #[test]
    fn test_h_index_5() {
        let citations = vec![100, 100];
        assert_eq!(Solution::h_index(citations), 2);
    }
}
