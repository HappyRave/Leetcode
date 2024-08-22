use crate::solutions::Solution;

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut mask = 1;
        while mask < num {
            mask = mask << 1 | 1;
        }
        !num & mask
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_complement() {
        assert_eq!(Solution::find_complement(5), 2);
    }

    #[test]
    fn test_find_complement_2() {
        assert_eq!(Solution::find_complement(1), 0);
    }
}
