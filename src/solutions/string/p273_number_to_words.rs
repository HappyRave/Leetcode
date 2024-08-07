use crate::solutions::Solution;

impl Solution {
    pub fn number_to_words(mut num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }
        const ORDERS: [&str; 4] = ["", "Thousand", "Million", "Billion"];
        const TENS: [&str; 10] = [
            "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
        ];
        const UNITS: [&str; 20] = [
            "",
            "One",
            "Two",
            "Three",
            "Four",
            "Five",
            "Six",
            "Seven",
            "Eight",
            "Nine",
            "Ten",
            "Eleven",
            "Twelve",
            "Thirteen",
            "Fourteen",
            "Fifteen",
            "Sixteen",
            "Seventeen",
            "Eighteen",
            "Nineteen",
        ];

        let mut result = String::new();
        let mut order = 0;
        while num > 0 {
            let mut current = num % 1000;
            num /= 1000;
            let mut current_result = String::new();
            if current > 0 {
                if current >= 100 {
                    current_result = UNITS[(current / 100) as usize].to_string() + " Hundred";
                    current %= 100;
                }
                if current >= 20 {
                    if !current_result.is_empty() {
                        current_result.push(' ');
                    }
                    current_result += TENS[(current / 10) as usize];
                    current %= 10;
                }
                if current > 0 {
                    if !current_result.is_empty() {
                        current_result.push(' ');
                    }
                    current_result += UNITS[current as usize];
                }
                if order > 0 && !current_result.is_empty() {
                    current_result.push(' ');
                    current_result += ORDERS[order];
                }
                result = current_result + " " + &result;
            }
            order += 1;
        }
        result.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_to_words() {
        assert_eq!(
            Solution::number_to_words(123),
            "One Hundred Twenty Three".to_string()
        );
    }

    #[test]
    fn test_number_to_words_2() {
        assert_eq!(
            Solution::number_to_words(12345),
            "Twelve Thousand Three Hundred Forty Five".to_string()
        );
    }

    #[test]
    fn test_number_to_words_3() {
        assert_eq!(
            Solution::number_to_words(1234567),
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven".to_string()
        );
    }

    #[test]
    fn test_number_to_words_4() {
        assert_eq!(Solution::number_to_words(1234567891), "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One".to_string());
    }

    #[test]
    fn test_number_to_words_5() {
        assert_eq!(
            Solution::number_to_words(1001),
            "One Thousand One".to_string()
        );
    }
}
