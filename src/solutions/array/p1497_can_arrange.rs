use crate::solutions::Solution;

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut count = vec![0; k as usize];
        for num in arr {
            count[((num % k + k) % k) as usize] += 1;
        }
        if count[0] % 2 != 0 {
            return false;
        }
        for i in 1..=k / 2 {
            if count[i as usize] != count[(k - i) as usize] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_arrange() {
        let arr = vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9];
        let k = 5;
        assert!(Solution::can_arrange(arr, k));
    }

    #[test]
    fn test_can_arrange_2() {
        let arr = vec![1, 2, 3, 4, 5, 6];
        let k = 7;
        assert!(Solution::can_arrange(arr, k));
    }

    #[test]
    fn test_can_arrange_3() {
        let arr = vec![1, 2, 3, 4, 5, 6];
        let k = 10;
        assert!(!Solution::can_arrange(arr, k));
    }

    #[test]
    fn test_can_arrange_4() {
        let arr = vec![-1, -1, -1, -1, 2, 2, -2, -2];
        let k = 3;
        assert!(!Solution::can_arrange(arr, k));
    }
}
