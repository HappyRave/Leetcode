use crate::solutions::Solution;

impl Solution {
    pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
        fn backtrack(nums: &[i32], k: i32, start: usize, cache: &mut Vec<Vec<Option<f64>>>) -> f64 {
            if cache[start][k as usize].is_some() {
                return cache[start][k as usize].unwrap();
            }
            if k == 1 {
                let sum: f64 = nums[start..].iter().sum::<i32>() as f64;
                let avg = sum / (nums.len() - start) as f64;
                cache[start][k as usize] = Some(avg);
                return avg;
            }
            let mut sum = 0;
            let mut max_avg: f64 = 0.0;
            for i in start..nums.len() - k as usize + 1 {
                sum += nums[i];
                let avg = sum as f64 / (i - start + 1) as f64;
                let next_avg = backtrack(nums, k - 1, i + 1, cache);
                max_avg = max_avg.max(avg + next_avg);
            }
            cache[start][k as usize] = Some(max_avg);
            max_avg
        }

        let mut cache = vec![vec![None; k as usize + 1]; nums.len()];
        backtrack(&nums, k, 0, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_sum_of_averages() {
        assert_eq!(
            Solution::largest_sum_of_averages(vec![9, 1, 2, 3, 9], 3),
            20.0
        );
    }

    #[test]
    fn test_largest_sum_of_averages_2() {
        assert_eq!(
            Solution::largest_sum_of_averages(vec![1, 2, 3, 4, 5, 6, 7], 4),
            20.5
        );
    }
}
