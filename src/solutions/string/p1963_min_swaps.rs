use crate::solutions::Solution;

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut stack_size = 0;
        for c in s.bytes() {
            if c == b'[' {
                stack_size += 1;
            } else if stack_size > 0 {
                stack_size -= 1;
            }
        }
        (stack_size + 1) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_swaps() {
        let s = "][][".to_string();
        let result = 1;
        assert_eq!(Solution::min_swaps(s), result);
    }

    #[test]
    fn test_min_swaps_2() {
        let s = "]]][[[".to_string();
        let result = 2;
        assert_eq!(Solution::min_swaps(s), result);
    }

    #[test]
    fn test_min_swaps_3() {
        let s = "[]".to_string();
        let result = 0;
        assert_eq!(Solution::min_swaps(s), result);
    }
}
