use crate::solutions::Solution;

impl Solution {
    pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        let mut sorted = happiness.clone();
        sorted.sort_unstable_by(|a, b| b.cmp(a));
        let mut sum = 0;
        let size = std::cmp::min(sorted.len(), k as usize);
        let mut index = 0;
        while index < size && sorted[index] > index as i32 {
            sum += (sorted[index] - index as i32) as i64;
            index += 1;
        }
        sum
    }

    pub fn maximum_happiness_sum_vector(happiness: Vec<i32>, k: i32) -> i64 {
        let mut sorted = happiness.clone();
        sorted.sort_unstable_by(|a, b| b.cmp(a));
        sorted
            .into_iter()
            .enumerate()
            .filter(|(i, h)| *h > *i as i32)
            .take(k as usize)
            .fold(0, |acc, (i, h)| acc + (h - i as i32) as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_happiness_sum() {
        let happiness = vec![1, 2, 3];
        let k = 2;
        let result = Solution::maximum_happiness_sum(happiness, k);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_maximum_happiness_sum_2() {
        let happiness = vec![1, 1, 1, 1];
        let k = 2;
        let result = Solution::maximum_happiness_sum(happiness, k);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_maximum_happiness_sum_3() {
        let happiness = vec![2, 3, 4, 5];
        let k = 1;
        let result = Solution::maximum_happiness_sum(happiness, k);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_maximum_happiness_sum_4() {
        let happiness = vec![2, 3, 4, 5];
        let k = 5;
        let result = Solution::maximum_happiness_sum(happiness, k);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_maximum_happiness_sum_vector() {
        let happiness = vec![1, 2, 3];
        let k = 2;
        let result = Solution::maximum_happiness_sum_vector(happiness, k);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_maximum_happiness_sum_vector_2() {
        let happiness = vec![1, 1, 1, 1];
        let k = 2;
        let result = Solution::maximum_happiness_sum_vector(happiness, k);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_maximum_happiness_sum_vector_3() {
        let happiness = vec![2, 3, 4, 5];
        let k = 1;
        let result = Solution::maximum_happiness_sum_vector(happiness, k);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_maximum_happiness_sum_vector_4() {
        let happiness = vec![2, 3, 4, 5];
        let k = 5;
        let result = Solution::maximum_happiness_sum_vector(happiness, k);
        assert_eq!(result, 9);
    }
}
