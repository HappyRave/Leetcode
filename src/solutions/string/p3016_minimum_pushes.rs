use crate::solutions::Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        const A: u8 = b'a';
        const ALPHABET_SIZE: usize = 26;
        const KEYS: usize = 8;
        let mut counts = [0; 26];
        for c in word.bytes() {
            counts[(c - A) as usize] += 1;
        }
        counts.sort_unstable_by(|a, b| b.cmp(a));
        let mut result = 0;
        for (i, c) in counts.iter().enumerate() {
            result += (i / KEYS + 1) * c;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_pushes() {
        assert_eq!(Solution::minimum_pushes("abcde".to_string()), 5);
    }

    #[test]
    fn test_minimum_pushes_2() {
        assert_eq!(Solution::minimum_pushes("xyzxyzxyzxyz".to_string()), 12); // cspell: disable-line
    }

    #[test]
    fn test_minimum_pushes_3() {
        assert_eq!(
            Solution::minimum_pushes("aabbccddeeffgghhiiiiii".to_string()), // cspell: disable-line
            24
        );
    }
}
