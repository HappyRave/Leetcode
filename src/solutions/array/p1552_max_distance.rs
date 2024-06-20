use crate::solutions::Solution;

impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        position.sort_unstable();
        let mut min = 1;
        let mut max = *position.last().unwrap() - position[0] + 1;
        let m = m as i64;

        while min < max {
            let mid = min + (max - min) / 2;
            let mut count = 1;
            let mut last = position[0];

            for &p in position.iter().skip(1) {
                if p - last >= mid {
                    count += 1;
                    last = p;
                }
            }

            if count < m {
                max = mid;
            } else {
                min = mid + 1;
            }
        }

        min - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_distance() {
        assert_eq!(Solution::max_distance(vec![1, 2, 3, 4, 7], 3), 3);
    }

    #[test]
    fn test_max_distance_2() {
        assert_eq!(
            Solution::max_distance(vec![5, 4, 3, 2, 1, 1000000000], 2),
            999999999
        );
    }
}
