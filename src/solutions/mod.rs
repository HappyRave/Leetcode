pub mod array;
pub mod binary_search;
pub mod binary_tree;
pub mod graph;
pub mod linked_lists;
pub mod math;
pub mod matrix;
pub mod misc;
pub mod simulation;
pub mod sorting;
pub mod stack;
pub mod string;
pub mod subset;
pub mod tree;

pub struct Solution;

trait ArrayStringExt {
    fn to_vec(self) -> Vec<i32>;
    fn to_matrix(self) -> Vec<Vec<i32>>;
    fn remove_matching_brackets(self) -> Self;
    fn split_matrix(&self) -> impl Iterator<Item = String>;
    fn to_string_vec(self) -> Vec<String>;
    fn to_char_vec(self) -> Vec<char>;
}

impl ArrayStringExt for &str {
    fn to_vec(self) -> Vec<i32> {
        self.remove_matching_brackets()
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect()
    }

    fn to_matrix(self) -> Vec<Vec<i32>> {
        self.split_matrix().map(|s| s.to_vec()).collect()
    }

    fn remove_matching_brackets(self) -> Self {
        if self.starts_with('[') && self.ends_with(']') {
            &self[1..self.len() - 1]
        } else {
            self
        }
    }

    fn split_matrix(&self) -> impl Iterator<Item = String> {
        self.remove_matching_brackets()
            .split("],")
            .filter(|s| !s.is_empty())
            .map(move |s| {
                if !s.ends_with(']') {
                    format!("{}]", s)
                } else {
                    s.to_string()
                }
            })
    }

    fn to_string_vec(self) -> Vec<String> {
        self.remove_matching_brackets()
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim_matches('"').to_string())
            .collect()
    }

    fn to_char_vec(self) -> Vec<char> {
        self.remove_matching_brackets()
            .split(',')
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.trim_matches('"').chars().next())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_string_ext() {
        let s = "[1,2,3,4,5]";
        let result = s.to_vec();
        let expected = vec![1, 2, 3, 4, 5];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_array_string_ext_2() {
        let s = "[]";
        let result = s.to_vec();
        let expected = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_array_string_ext_3() {
        let s = "[[1,2,3],[4,5,6],[7,8,9]]";
        let result = s.to_matrix();
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_array_string_ext_4() {
        let s = "[]";
        let result = s.to_matrix();
        let expected: Vec<Vec<_>> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_array_string_ext_5() {
        let s = "[[],[]]";
        let result = s.to_matrix();
        let expected = vec![vec![], vec![]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_remove_matching_brackets() {
        let s = "[1,2,3]";
        let result = s.remove_matching_brackets();
        let expected = "1,2,3";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_remove_matching_brackets_2() {
        let s = "1,2,3";
        let result = s.remove_matching_brackets();
        let expected = "1,2,3";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_remove_matching_brackets_3() {
        let s = "";
        let result = s.remove_matching_brackets();
        let expected = "";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_matrix() {
        let s = "[[1,2,3],[4,5,6],[7,8,9]]";
        let result = s.split_matrix().collect::<Vec<_>>();
        let expected = vec!["[1,2,3]", "[4,5,6]", "[7,8,9]"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_string_vec() {
        let s = "[\"a\",\"b\",\"c\",\"d\"]";
        let result = s.to_string_vec();
        let expected = vec!["a", "b", "c", "d"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_string_vec_2() {
        let s = "[\"alice\"]";
        let result = s.to_string_vec();
        let expected = vec!["alice"];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_string_vec_3() {
        let s = "[]";
        let result = s.to_string_vec();
        let expected: Vec<String> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_char_vec() {
        let s = "[\"a\",\"b\",\"c\",\"d\"]";
        let result = s.to_char_vec();
        let expected = vec!['a', 'b', 'c', 'd'];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_char_vec_2() {
        let s = "[\"a\"]";
        let result = s.to_char_vec();
        let expected = vec!['a'];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_char_vec_3() {
        let s = "[]";
        let result = s.to_char_vec();
        let expected: Vec<char> = vec![];
        assert_eq!(result, expected);
    }
}
