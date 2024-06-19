use crate::solutions::Solution;

impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        if m as i64 * k as i64 > bloom_day.len() as i64 {
            return -1;
        }

        let mut left = 1;
        let mut right = *bloom_day.iter().max().unwrap();

        while left < right {
            let mid = left + (right - left) / 2;
            let mut bouquets = 0;
            let mut flowers = 0;

            for day in bloom_day.iter() {
                if *day <= mid {
                    flowers += 1;
                    if flowers == k {
                        bouquets += 1;
                        flowers = 0;
                    }
                } else {
                    flowers = 0;
                }
            }

            if bouquets < m {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_days() {
        assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 1), 3);
    }

    #[test]
    fn test_min_days_2() {
        assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 2), -1);
    }

    #[test]
    fn test_min_days_3() {
        assert_eq!(Solution::min_days(vec![7, 7, 7, 7, 12, 7, 7], 2, 3), 12);
    }

    #[test]
    fn test_min_days_4() {
        assert_eq!(
            Solution::min_days(vec![1000000000, 1000000000], 1, 1),
            1000000000
        );
    }

    #[test]
    fn test_min_days_5() {
        assert_eq!(
            Solution::min_days(vec![1, 10, 2, 9, 3, 8, 4, 7, 5, 6], 4, 2),
            9
        );
    }
}
