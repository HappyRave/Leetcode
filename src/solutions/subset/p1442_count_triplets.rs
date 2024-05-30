use crate::solutions::Solution;

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut res = 0;
        let n = arr.len();
        for i in 0..n {
            let mut sum = arr[i];
            for j in (i + 1)..n {
                sum ^= arr[j];
                if sum == 0 {
                    res += j - i;
                }
            }
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let arr = vec![2, 3, 1, 6, 7];
        let res = 4;
        assert_eq!(Solution::count_triplets(arr), res);
    }

    #[test]
    fn test_2() {
        let arr = vec![1, 1, 1, 1, 1];
        let res = 10;
        assert_eq!(Solution::count_triplets(arr), res);
    }

    #[test]
    fn test_3() {
        let arr = vec![2, 3];
        let res = 0;
        assert_eq!(Solution::count_triplets(arr), res);
    }

    #[test]
    fn test_4() {
        let arr = vec![1, 3, 5, 7, 9];
        let res = 3;
        assert_eq!(Solution::count_triplets(arr), res);
    }

    #[test]
    fn test_5() {
        let arr = vec![7, 11, 12, 9, 5, 2, 7, 17, 22];
        let res = 8;
        assert_eq!(Solution::count_triplets(arr), res);
    }
}
