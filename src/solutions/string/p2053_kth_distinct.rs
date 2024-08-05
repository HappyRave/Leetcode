use crate::solutions::Solution;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut map = std::collections::HashMap::new();
        for s in &arr {
            let entry = map.entry(s).or_insert(0);
            *entry += 1;
        }
        let mut counter = 0;
        for s in &arr {
            if let Some(count) = map.get(s) {
                if *count == 1 {
                    counter += 1;
                }
                if counter == k {
                    return s.to_string();
                }
            }
        }
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::ArrayStringExt;

    use super::*;

    #[test]
    fn test_kth_distinct() {
        let arr = "[\"d\",\"b\",\"c\",\"b\",\"c\",\"a\"]".to_string_vec();
        let k = 2;
        let result = "a".to_string();
        assert_eq!(Solution::kth_distinct(arr, k), result);
    }

    #[test]
    fn test_kth_distinct_2() {
        let arr = "[\"aaa\",\"aa\",\"a\"]".to_string_vec();
        let k = 1;
        let result = "aaa".to_string();
        assert_eq!(Solution::kth_distinct(arr, k), result);
    }

    #[test]
    fn test_kth_distinct_3() {
        let arr = "[\"a\",\"b\",\"a\"]".to_string_vec();
        let k = 3;
        let result = "".to_string();
        assert_eq!(Solution::kth_distinct(arr, k), result);
    }
}
