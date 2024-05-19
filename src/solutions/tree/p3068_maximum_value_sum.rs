use crate::solutions::Solution;

impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, _edges: Vec<Vec<i32>>) -> i64 {
        let mut sum: i64 = 0;
        let mut count_changed = 0;
        let mut min_diff = i32::MAX;
        let mut min_changed = 0;

        for num in nums {
            let changed = num ^ k;
            let diff = (num - changed).abs();
            let add;
            if changed < num {
                add = num;
            } else {
                add = changed;
                count_changed += 1;
            }

            sum += add as i64;

            if diff < min_diff {
                min_diff = diff;
                min_changed = add;
            }
        }

        if count_changed % 2 == 0 {
            sum
        } else {
            sum -= min_changed as i64;
            sum += (min_changed ^ k) as i64;
            sum
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_value_sum() {
        let nums = vec![1, 2, 1];
        let k = 3;
        let edges = vec![vec![0, 1], vec![1, 2]];
        assert_eq!(Solution::maximum_value_sum(nums, k, edges), 6);
    }

    #[test]
    fn test_maximum_value_sum_2() {
        let nums = vec![2, 3];
        let k = 7;
        let edges = vec![vec![0, 1]];
        assert_eq!(Solution::maximum_value_sum(nums, k, edges), 9);
    }

    #[test]
    fn test_maximum_value_sum_3() {
        let nums = vec![7, 7, 7, 7, 7, 7];
        let k = 3;
        let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5]];
        assert_eq!(Solution::maximum_value_sum(nums, k, edges), 42);
    }

    #[test]
    fn test_maximum_value_sum_4() {
        let nums = vec![24, 78, 1, 97, 44];
        let k = 6;
        let edges = vec![vec![0, 2], vec![1, 2], vec![4, 2], vec![3, 4]];
        assert_eq!(Solution::maximum_value_sum(nums, k, edges), 260);
    }
}
