use crate::solutions::Solution;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut max = 0;
        for (i, n) in arr.into_iter().enumerate() {
            max = max.max(n);
            if i as i32 == max {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_chunks_to_sorted() {
        assert_eq!(Solution::max_chunks_to_sorted(vec![4, 3, 2, 1, 0]), 1);
    }

    #[test]
    fn test_max_chunks_to_sorted_2() {
        assert_eq!(Solution::max_chunks_to_sorted(vec![1, 0, 2, 3, 4]), 4);
    }
}
