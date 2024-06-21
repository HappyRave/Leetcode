use crate::solutions::Solution;

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let mut sum = 0;
        let mut max = 0;
        let mut current = 0;
        let mut i = 0;
        for j in 0..customers.len() {
            sum += customers[j] * (1 - grumpy[j]);
            current += customers[j] * grumpy[j];
            if j - i + 1 > minutes as usize {
                current -= customers[i] * grumpy[i];
                i += 1;
            }
            max = max.max(current);
        }
        sum + max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_satisfied() {
        assert_eq!(
            Solution::max_satisfied(
                vec![1, 0, 1, 2, 1, 1, 7, 5],
                vec![0, 1, 0, 1, 0, 1, 0, 1],
                3
            ),
            16
        );
    }

    #[test]
    fn test_max_satisfied_2() {
        assert_eq!(Solution::max_satisfied(vec![1], vec![0], 1), 1);
    }

    #[test]
    fn test_max_satisfied_3() {
        assert_eq!(Solution::max_satisfied(vec![1], vec![1], 1), 1);
    }

    #[test]
    fn test_max_satisfied_4() {
        assert_eq!(Solution::max_satisfied(vec![1, 1], vec![1, 1], 1), 1);
    }
}
