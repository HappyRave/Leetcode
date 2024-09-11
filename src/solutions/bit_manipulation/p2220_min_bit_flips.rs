use crate::solutions::Solution;

impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        let (mut mask, mut flips) = (1, 0);
        while mask <= start || mask <= goal {
            if start & mask != goal & mask {
                flips += 1;
            }
            mask <<= 1;
        }
        flips
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_bit_flips() {
        let start = 10;
        let goal = 7;
        let res = 3;
        assert_eq!(Solution::min_bit_flips(start, goal), res);
    }

    #[test]
    fn test_min_bit_flips_2() {
        let start = 3;
        let goal = 4;
        let res = 3;
        assert_eq!(Solution::min_bit_flips(start, goal), res);
    }
}
