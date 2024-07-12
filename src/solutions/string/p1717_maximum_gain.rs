use crate::solutions::Solution;

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let (first, second, x, y) = if x > y {
            ((b'a', b'b'), (b'b', b'a'), x, y)
        } else {
            ((b'b', b'a'), (b'a', b'b'), y, x)
        };

        let mut s = s.into_bytes();
        let mut res = 0;

        fn process_pattern(s: &mut Vec<u8>, pattern: (u8, u8), score: i32, resize: bool) -> i32 {
            let mut total = 0;
            let mut pos = 0;
            for i in 0..s.len() {
                s[pos] = s[i];
                pos += 1;
                if pos > 1 && (s[pos - 2], s[pos - 1]) == pattern {
                    pos -= 2;
                    total += score;
                }
            }
            if resize {
                s.resize(pos, 0);
            }
            total
        }

        res += process_pattern(&mut s, first, x, true);
        res += process_pattern(&mut s, second, y, false);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_gain() {
        let s = "cdbcbbaaabab".to_string(); // cspell: disable-line
        let x = 4;
        let y = 5;
        let res = 19;
        assert_eq!(Solution::maximum_gain(s, x, y), res);
    }

    #[test]
    fn test_maximum_gain_2() {
        let s = "aabbaaxybbaabb".to_string(); // cspell: disable-line
        let x = 5;
        let y = 4;
        let res = 20;
        assert_eq!(Solution::maximum_gain(s, x, y), res);
    }

    #[test]
    fn test_maximum_gain_3() {
        let s = "a".to_string();
        let x = 1;
        let y = 1;
        let res = 0;
        assert_eq!(Solution::maximum_gain(s, x, y), res);
    }
}
