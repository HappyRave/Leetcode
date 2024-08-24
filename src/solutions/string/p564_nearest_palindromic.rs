use crate::solutions::Solution;

impl Solution {
    pub fn nearest_palindromic(n: String) -> String {
        if n.is_empty() {
            return "".to_string();
        }

        let len = n.len();
        let n = n.parse::<i64>().unwrap();

        if n <= 10 {
            return (n - 1).to_string();
        }

        if n == 11 {
            return "9".to_string();
        }

        fn get_palindrome(n: i64) -> i64 {
            let s = n.to_string();
            let mut chars: Vec<char> = s.chars().collect();
            let mut i = 0;
            let mut j = chars.len() - 1;
            while i < j {
                chars[j] = chars[i];
                i += 1;
                j -= 1;
            }
            chars
                .into_iter()
                .collect::<String>()
                .parse::<i64>()
                .unwrap()
        }

        let mut res = 0;
        let mut diff = i64::max_value();
        for i in 0..(len / 2) + 1 {
            for j in -1..=1 {
                let palindrome = get_palindrome(n + j * 10_i64.pow(i as u32));
                if palindrome != n {
                    let d = (palindrome - n).abs();
                    if d < diff || (d == diff && palindrome < res) {
                        diff = d;
                        res = palindrome;
                    }
                }
            }
        }
        res.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nearest_palindromic() {
        assert_eq!(
            Solution::nearest_palindromic("".to_string()),
            "".to_string()
        );
    }

    #[test]
    fn test_nearest_palindromic_2() {
        assert_eq!(
            Solution::nearest_palindromic("1".to_string()),
            "0".to_string()
        );
    }

    #[test]
    fn test_nearest_palindromic_3() {
        assert_eq!(
            Solution::nearest_palindromic("9".to_string()),
            "8".to_string()
        );
    }

    #[test]
    fn test_nearest_palindromic_4() {
        assert_eq!(
            Solution::nearest_palindromic("123".to_string()),
            "121".to_string()
        );
    }

    #[test]
    fn test_nearest_palindromic_5() {
        assert_eq!(
            Solution::nearest_palindromic("230".to_string()),
            "232".to_string()
        );
    }

    #[test]
    fn test_nearest_palindromic_6() {
        assert_eq!(
            Solution::nearest_palindromic("88".to_string()),
            "77".to_string()
        );
    }

    #[test]
    fn test_nearest_palindromic_7() {
        assert_eq!(
            Solution::nearest_palindromic("11911".to_string()),
            "11811".to_string()
        );
    }
}
