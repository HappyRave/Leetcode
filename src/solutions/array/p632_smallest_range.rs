use crate::solutions::Solution;

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut numbers = vec![];
        for (i, num) in nums.iter().enumerate() {
            for &n in num.iter() {
                numbers.push((n, i));
            }
        }

        numbers.sort_by_key(|x| x.0);

        let mut count: Vec<usize> = vec![0; nums.len()];
        let mut i: usize = 0;
        let mut k: usize = 0;

        let mut lower_bound: i32 = -100001;
        let mut upper_bound: i32 = 100001;
        for (n, (num, idx)) in numbers.iter().enumerate() {
            count[*idx] += 1;
            if count[*idx] == 1 {
                k += 1;
            }
            if k == nums.len() {
                while i <= n && count[numbers[i].1] > 1 {
                    count[numbers[i].1] -= 1;
                    i += 1;
                }
                if *num - numbers[i].0 < upper_bound - lower_bound {
                    upper_bound = *num;
                    lower_bound = numbers[i].0;
                }
            }
        }
        vec![lower_bound, upper_bound]
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::ArrayStringExt;

    use super::*;

    #[test]
    fn test_smallest_range() {
        let nums = "[[4,10,15,24,26],[0,9,12,20],[5,18,22,30]]".to_matrix();
        let res = vec![20, 24];
        assert_eq!(Solution::smallest_range(nums), res);
    }

    #[test]
    fn test_smallest_range_2() {
        let nums = "[[1,2,3],[1,2,3],[1,2,3]]".to_matrix();
        let res = vec![1, 1];
        assert_eq!(Solution::smallest_range(nums), res);
    }
}
