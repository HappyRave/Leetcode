use crate::solutions::Solution;

impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let mut heap = std::collections::BinaryHeap::from(
            nums.iter()
                .enumerate()
                .map(|(i, &num)| (std::cmp::Reverse(num), i))
                .collect::<Vec<_>>(),
        );
        let mut sum: i32 = 0;
        for i in 1..=right {
            match heap.pop() {
                Some((std::cmp::Reverse(num), idx)) => {
                    if i >= left {
                        sum = (sum + num) % 1_000_000_007;
                    }
                    if idx + 1 < n as usize {
                        heap.push((
                            std::cmp::Reverse((num + nums[idx + 1]) % 1_000_000_007),
                            idx + 1,
                        ));
                    }
                }
                None => break,
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_sum() {
        assert_eq!(Solution::range_sum(vec![1, 2, 3, 4], 4, 1, 5), 13);
    }

    #[test]
    fn test_range_sum_2() {
        assert_eq!(Solution::range_sum(vec![1, 2, 3, 4], 4, 3, 4), 6);
    }

    #[test]
    fn test_range_sum_3() {
        assert_eq!(Solution::range_sum(vec![1, 2, 3, 4], 4, 1, 10), 50);
    }

    #[test]
    fn test_range_sum_4() {
        assert_eq!(
            Solution::range_sum(vec![100; 1000], 1000, 1, 500500),
            716699888
        );
    }
}
