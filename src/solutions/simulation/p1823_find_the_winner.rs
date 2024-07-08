use crate::solutions::Solution;

impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut current = 0;
        for i in 2..=n {
            current = (current + k) % i;
        }
        current + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_the_winner() {
        assert_eq!(Solution::find_the_winner(5, 2), 3);
    }

    #[test]
    fn test_find_the_winner_2() {
        assert_eq!(Solution::find_the_winner(6, 5), 1);
    }
}
