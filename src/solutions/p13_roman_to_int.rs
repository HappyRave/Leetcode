use super::Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut last_num = 0;
        let roman_map: std::collections::HashMap<char, i32> = [
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .iter()
        .cloned()
        .collect();

        for c in s.chars().rev() {
            let num = *roman_map.get(&c).unwrap();
            if num < last_num {
                result -= num;
            } else {
                result += num;
            }
            last_num = num;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
