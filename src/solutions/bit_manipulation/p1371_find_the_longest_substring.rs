use crate::solutions::Solution;

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut ans = 0;
        let mut state = 0;
        let mut pos = [-1; 32];
        pos[0] = 0;
        for (i, c) in s.chars().enumerate() {
            match c {
                'a' => state ^= 1,
                'e' => state ^= 2,
                'i' => state ^= 4,
                'o' => state ^= 8,
                'u' => state ^= 16,
                _ => (),
            }
            if pos[state] == -1 {
                pos[state] = i as i32 + 1;
            } else {
                ans = ans.max(i as i32 + 1 - pos[state]);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_the_longest_substring() {
        assert_eq!(Solution::find_the_longest_substring("".to_string()), 0);
    }

    #[test]
    fn test_find_the_longest_substring_2() {
        assert_eq!(
            Solution::find_the_longest_substring("eleetminicoworoep".to_string()), // cspell: disable-line
            13
        );
    }

    #[test]
    fn test_find_the_longest_substring_3() {
        assert_eq!(
            Solution::find_the_longest_substring("leetcodeisgreat".to_string()), // cspell: disable-line
            5
        );
    }

    #[test]
    fn test_find_the_longest_substring_4() {
        assert_eq!(
            Solution::find_the_longest_substring("bcbcbc".to_string()), // cspell: disable-line
            6
        );
    }
}
