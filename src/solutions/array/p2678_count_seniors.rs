use crate::solutions::Solution;

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .iter()
            .filter(|&detail| detail[11..13].parse::<u8>().unwrap() > 60)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::ArrayStringExt;

    use super::*;

    #[test]
    fn test_count_seniors() {
        let details =
            "[\"7868190130M7522\",\"5303914400F9211\",\"9273338290F4010\"]".to_string_vec();
        assert_eq!(Solution::count_seniors(details), 2);
    }

    #[test]
    fn test_count_seniors_2() {
        let details = "[\"1313579440F2036\",\"2921522980M5644\"]".to_string_vec();
        assert_eq!(Solution::count_seniors(details), 0);
    }

    #[test]
    fn test_count_seniors_3() {
        let details = "[\"9751302862F0693\",\"3888560693F7262\",\"5485983835F0649\",\"2580974299F6042\",\"9976672161M6561\",\"0234451011F8013\",\"4294552179O6482\"]".to_string_vec();
        assert_eq!(Solution::count_seniors(details), 4);
    }
}
