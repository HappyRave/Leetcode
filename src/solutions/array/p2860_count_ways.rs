use crate::solutions::Solution;

impl Solution {
    pub fn count_ways(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        (0..=n).fold(0, |acc, i| {
            let mut left = 0;
            let mut right = n;
            while left < right {
                let mid = left + (right - left) / 2;
                match nums[mid].cmp(&(i as i32)) {
                    std::cmp::Ordering::Less => left = mid + 1,
                    std::cmp::Ordering::Greater => right = mid,
                    std::cmp::Ordering::Equal => return acc,
                }
            }
            if left != i {
                acc
            } else {
                acc + 1
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_ways() {
        let nums = vec![1, 1];
        let result = 2;
        assert_eq!(Solution::count_ways(nums), result);
    }

    #[test]
    fn test_count_ways_2() {
        let nums = vec![6, 0, 3, 3, 6, 7, 2, 7];
        let result = 3;
        assert_eq!(Solution::count_ways(nums), result);
    }
}
