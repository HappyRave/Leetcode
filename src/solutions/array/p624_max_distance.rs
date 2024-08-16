use crate::solutions::Solution;

impl Solution {
    pub fn max_distance_2(arrays: Vec<Vec<i32>>) -> i32 {
        let mut min = arrays[0][0];
        let mut max = arrays[0].last().unwrap();
        let mut result = 0;

        for array in arrays.iter().skip(1) {
            let current_min = array[0];
            let current_max = array.last().unwrap();
            result = result.max((current_max - min).abs());
            result = result.max((max - current_min).abs());
            min = min.min(current_min);
            max = max.max(current_max);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_distance_2() {
        let arrays = vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]];
        let result = 4;
        assert_eq!(Solution::max_distance_2(arrays), result);
    }

    #[test]
    fn test_max_distance_2_2() {
        let arrays = vec![vec![1], vec![1]];
        let result = 0;
        assert_eq!(Solution::max_distance_2(arrays), result);
    }

    #[test]
    fn test_max_distance_2_3() {
        let arrays = vec![vec![1, 4], vec![0, 5]];
        let result = 4;
        assert_eq!(Solution::max_distance_2(arrays), result);
    }
}
