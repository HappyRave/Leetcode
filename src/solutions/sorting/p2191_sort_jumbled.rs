use crate::solutions::Solution;

impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, mut nums: Vec<i32>) -> Vec<i32> {
        fn map(mut num: i32, mapping: &[i32]) -> i32 {
            if num == 0 {
                return mapping[0];
            }
            let mut mapped_num = 0;
            let mut multiplier = 1;
            while num > 0 {
                let digit = num % 10;
                num /= 10;
                mapped_num += mapping[digit as usize] * multiplier;
                multiplier *= 10;
            }
            mapped_num
        }
        nums.sort_by_cached_key(|&num| map(num, &mapping));
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_jumbled() {
        let mapping = vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6];
        let nums = vec![991, 338, 38];
        let res = Solution::sort_jumbled(mapping, nums);
        assert_eq!(res, vec![338, 38, 991]);
    }

    #[test]
    fn test_sort_jumbled_1() {
        let mapping = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let nums = vec![789, 456, 123];
        let res = Solution::sort_jumbled(mapping, nums);
        assert_eq!(res, vec![123, 456, 789]);
    }
}
