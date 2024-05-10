use crate::solutions::Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        for letter in &letters {
            if *letter > target {
                return *letter;
            }
        }
        letters[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_greatest_letter_a() {
        let letters = vec!['c', 'f', 'j'];
        let target = 'a';
        let result = Solution::next_greatest_letter(letters, target);
        assert_eq!(result, 'c');
    }

    #[test]
    fn test_next_greatest_letter_c() {
        let letters = vec!['c', 'f', 'j'];
        let target = 'c';
        let result = Solution::next_greatest_letter(letters, target);
        assert_eq!(result, 'f');
    }

    #[test]
    fn test_next_greatest_letter_d() {
        let letters = vec!['c', 'f', 'j'];
        let target = 'd';
        let result = Solution::next_greatest_letter(letters, target);
        assert_eq!(result, 'f');
    }

    #[test]
    fn test_next_greatest_letter_g() {
        let letters = vec!['c', 'f', 'j'];
        let target = 'g';
        let result = Solution::next_greatest_letter(letters, target);
        assert_eq!(result, 'j');
    }

    #[test]
    fn test_next_greatest_letter_j() {
        let letters = vec!['c', 'f', 'j'];
        let target = 'j';
        let result = Solution::next_greatest_letter(letters, target);
        assert_eq!(result, 'c');
    }

    #[test]
    fn test_next_greatest_letter_k() {
        let letters = vec!['c', 'f', 'j'];
        let target = 'k';
        let result = Solution::next_greatest_letter(letters, target);
        assert_eq!(result, 'c');
    }

    #[test]
    fn test_next_greatest_letter_g_with_duplicates() {
        let letters = vec!['e', 'e', 'g', 'g'];
        let target = 'g';
        let result = Solution::next_greatest_letter(letters, target);
        assert_eq!(result, 'e');
    }
}
