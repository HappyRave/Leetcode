use crate::solutions::Solution;

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut n = n;
        let mut max_gap = 0;
        let mut gap = 0;
        let mut found_one = false;
        while n > 0 {
            if n & 1 == 1 {
                if found_one {
                    max_gap = max_gap.max(gap);
                }
                gap = 1;
                found_one = true;
            } else if found_one {
                gap += 1;
            }
            n >>= 1;
        }
        max_gap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_gap() {
        assert_eq!(Solution::binary_gap(22), 2);
    }

    #[test]
    fn test_binary_gap_2() {
        assert_eq!(Solution::binary_gap(8), 0);
    }

    #[test]
    fn test_binary_gap_3() {
        assert_eq!(Solution::binary_gap(5), 2);
    }
}
