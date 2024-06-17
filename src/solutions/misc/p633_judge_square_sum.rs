use crate::solutions::Solution;

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let c = c as i64;
        let mut left = 0;
        let mut right = (c as f64).sqrt() as i64;
        while left <= right {
            let sum: i64 = left * left + right * right;

            match sum.cmp(&c) {
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Greater => right -= 1,
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_judge_square_sum() {
        assert!(Solution::judge_square_sum(5));
    }

    #[test]
    fn test_judge_square_sum_2() {
        assert!(!Solution::judge_square_sum(3));
    }

    #[test]
    fn test_judge_square_sum_3() {
        assert!(Solution::judge_square_sum(4));
    }

    #[test]
    fn test_judge_square_sum_4() {
        assert!(Solution::judge_square_sum(2));
    }

    #[test]
    fn test_judge_square_sum_5() {
        assert!(Solution::judge_square_sum(1));
    }

    #[test]
    fn test_judge_square_sum_6() {
        assert!(Solution::judge_square_sum(2147483600));
    }
}
