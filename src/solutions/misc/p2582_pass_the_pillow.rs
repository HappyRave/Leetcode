use crate::solutions::Solution;

impl Solution {
    pub fn pass_the_pillow(n: i32, mut time: i32) -> i32 {
        let round_trip = n * 2 - 2;
        time %= round_trip;
        let r = time % n;
        if time / n % 2 == 0 {
            r + 1
        } else {
            n - r - 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_the_pillow() {
        let n = 4;
        let time = 5;
        let result = 2;
        assert_eq!(Solution::pass_the_pillow(n, time), result);
    }

    #[test]
    fn test_pass_the_pillow_2() {
        let n = 3;
        let time = 2;
        let result = 3;
        assert_eq!(Solution::pass_the_pillow(n, time), result);
    }

    #[test]
    fn test_pass_the_pillow_3() {
        let n = 10;
        let time = 5;
        let result = 6;
        assert_eq!(Solution::pass_the_pillow(n, time), result);
    }

    #[test]
    fn test_pass_the_pillow_4() {
        let n = 10;
        let time = 10;
        let result = 9;
        assert_eq!(Solution::pass_the_pillow(n, time), result);
    }

    #[test]
    fn test_pass_the_pillow_5() {
        let n = 10;
        let time = 15;
        let result = 4;
        assert_eq!(Solution::pass_the_pillow(n, time), result);
    }

    #[test]
    fn test_pass_the_pillow_6() {
        let n = 18;
        let time = 38;
        let result = 5;
        assert_eq!(Solution::pass_the_pillow(n, time), result);
    }
}
