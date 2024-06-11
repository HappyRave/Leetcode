use crate::solutions::Solution;

impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut count = vec![0; 1001];
        let mut result = Vec::new();
        for num in arr1 {
            count[num as usize] += 1;
        }
        for num in arr2 {
            for _ in 0..count[num as usize] {
                result.push(num);
            }
            count[num as usize] = 0;
        }
        for (num, &c) in count.iter().enumerate() {
            for _ in 0..c {
                result.push(num as i32);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_sort_array() {
        let arr1 = vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19];
        let arr2 = vec![2, 1, 4, 3, 9, 6];
        let result = vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19];
        assert_eq!(Solution::relative_sort_array(arr1, arr2), result);
    }

    #[test]
    fn test_relative_sort_array_2() {
        let arr1 = vec![28, 6, 22, 8, 44, 17];
        let arr2 = vec![22, 28, 8, 6];
        let result = vec![22, 28, 8, 6, 17, 44];
        assert_eq!(Solution::relative_sort_array(arr1, arr2), result);
    }
}
