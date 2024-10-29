use crate::solutions::Solution;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut ranks = arr.clone();
        ranks.sort_unstable();
        ranks.dedup();

        let ranks = ranks
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<std::collections::HashMap<i32, usize>>();

        arr.iter().map(|&x| ranks[&x] as i32 + 1).collect()
    }

    pub fn array_rank_transform_2(arr: Vec<i32>) -> Vec<i32> {
        let mut sorted_map = arr.iter().enumerate().collect::<Vec<_>>();
        sorted_map.sort_unstable_by_key(|&(_, v)| v);
        let mut rank = 0;
        let mut prev = i32::MIN;
        let mut result = vec![0; arr.len()];
        for (i, &v) in sorted_map.iter() {
            if v != prev {
                rank += 1;
            }
            prev = v;
            result[*i] = rank;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_rank_transform() {
        let arr = vec![40, 10, 20, 30];
        let result = vec![4, 1, 2, 3];
        assert_eq!(Solution::array_rank_transform(arr), result);
    }

    #[test]
    fn test_array_rank_transform_2() {
        let arr = vec![-40, 10, -20, 30];
        let result = vec![1, 3, 2, 4];
        assert_eq!(Solution::array_rank_transform(arr), result);
    }

    #[test]
    fn test_array_rank_transform_3() {
        let arr = vec![100, 100, 100];
        let result = vec![1, 1, 1];
        assert_eq!(Solution::array_rank_transform(arr), result);
    }

    #[test]
    fn test_array_rank_transform_4() {
        let arr = vec![37, 12, 28, 9, 100, 56, 80, 5, 12];
        let result = vec![5, 3, 4, 2, 8, 6, 7, 1, 3];
        assert_eq!(Solution::array_rank_transform(arr), result);
    }
}
