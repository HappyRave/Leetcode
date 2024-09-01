use crate::solutions::Solution;

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let len = original.len() as i32;
        if len != m * n {
            return vec![];
        }
        let mut res = vec![vec![0; n as usize]; m as usize];
        for i in 0..len {
            let row = i / n;
            let col = i % n;
            res[row as usize][col as usize] = original[i as usize];
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct2_d_array() {
        let original = vec![1, 2, 3, 4];
        let m = 2;
        let n = 2;
        let res = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(Solution::construct2_d_array(original, m, n), res);
    }

    #[test]
    fn test_construct2_d_array_2() {
        let original = vec![1, 2, 3];
        let m = 1;
        let n = 3;
        let res = vec![vec![1, 2, 3]];
        assert_eq!(Solution::construct2_d_array(original, m, n), res);
    }

    #[test]
    fn test_construct2_d_array_3() {
        let original = vec![1, 2];
        let m = 1;
        let n = 1;
        let res: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::construct2_d_array(original, m, n), res);
    }
}
