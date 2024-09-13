use crate::solutions::Solution;

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut xor = vec![0; arr.len() + 1];
        for i in 0..arr.len() {
            xor[i + 1] = xor[i] ^ arr[i];
        }
        queries
            .iter()
            .map(|q| xor[q[0] as usize] ^ xor[q[1] as usize + 1])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_queries() {
        let arr = vec![1, 3, 4, 8];
        let queries = vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]];
        let res = vec![2, 7, 14, 8];
        assert_eq!(Solution::xor_queries(arr, queries), res);
    }

    #[test]
    fn test_xor_queries_2() {
        let arr = vec![4, 8, 2, 10];
        let queries = vec![vec![2, 3], vec![1, 3], vec![0, 0], vec![0, 3]];
        let res = vec![8, 0, 4, 4];
        assert_eq!(Solution::xor_queries(arr, queries), res);
    }
}
