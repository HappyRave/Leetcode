use crate::solutions::Solution;

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut num = num;
        let mut digits = Vec::new();
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        let mut digits = digits.iter().rev().collect::<Vec<_>>();
        let mut max = digits.clone();
        for i in 0..digits.len() {
            for j in (i + 1)..digits.len() {
                digits.swap(i, j);
                if digits > max {
                    max.clone_from(&digits);
                }
                digits.swap(i, j);
            }
        }
        let mut result = 0;
        for digit in max {
            result = result * 10 + digit;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_swap() {
        assert_eq!(Solution::maximum_swap(2736), 7236);
    }

    #[test]
    fn test_maximum_swap_2() {
        assert_eq!(Solution::maximum_swap(9973), 9973);
    }
}
