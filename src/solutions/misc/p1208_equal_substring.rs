use crate::solutions::Solution;

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        fn swap_cost(c1: char, c2: char) -> i32 {
            (c1 as i32 - c2 as i32).abs()
        }

        let s = s.chars().collect::<Vec<_>>();
        let t = t.chars().collect::<Vec<_>>();

        let mut left = 0;
        let mut right = 0;
        let mut cost = 0;
        let mut max_len = 0;

        while right < s.len() {
            cost += swap_cost(s[right], t[right]);
            right += 1;

            while cost > max_cost {
                cost -= swap_cost(s[left], t[left]);
                left += 1;
            }

            max_len = max_len.max(right - left);
        }
        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1208_equal_substring_1() {
        assert_eq!(
            Solution::equal_substring("abcd".to_string(), "bcdf".to_string(), 3),
            3
        );
    }

    #[test]
    fn p1208_equal_substring_2() {
        assert_eq!(
            Solution::equal_substring("abcd".to_string(), "cdef".to_string(), 3),
            1
        );
    }

    #[test]
    fn p1208_equal_substring_3() {
        assert_eq!(
            Solution::equal_substring("abcd".to_string(), "acde".to_string(), 0),
            1
        );
    }
}
