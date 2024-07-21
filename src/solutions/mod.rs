pub mod array;
pub mod binary_search;
pub mod binary_tree;
pub mod graph;
pub mod linked_lists;
pub mod matrix;
pub mod misc;
pub mod simulation;
pub mod stack;
pub mod string;
pub mod subset;
pub mod tree;

pub struct Solution;

trait ArrayStringExt {
    fn to_vec(self) -> Vec<i32>;
    fn to_matrix(self) -> Vec<Vec<i32>>;
}

impl ArrayStringExt for &str {
    fn to_vec(self) -> Vec<i32> {
        remove_matching_brackets(self)
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect()
    }

    fn to_matrix(self) -> Vec<Vec<i32>> {
        split_matrix(self).iter().map(|s| s.to_vec()).collect()
    }
}

fn remove_matching_brackets(s: &str) -> &str {
    if s.starts_with('[') && s.ends_with(']') {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

fn split_matrix(s: &str) -> Vec<String> {
    remove_matching_brackets(s)
        .split("],")
        .filter(|s| !s.is_empty())
        .map(|s| {
            if !s.ends_with(']') {
                format!("{}]", s)
            } else {
                s.to_string()
            }
        })
        .collect()
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
        let result = remove_matching_brackets(s);
        let expected = "1,2,3";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_remove_matching_brackets_2() {
        let s = "1,2,3";
        let result = remove_matching_brackets(s);
        let expected = "1,2,3";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_remove_matching_brackets_3() {
        let s = "";
        let result = remove_matching_brackets(s);
        let expected = "";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_matrix() {
        let s = "[[1,2,3],[4,5,6],[7,8,9]]";
        let result = split_matrix(s);
        let expected = vec!["[1,2,3]", "[4,5,6]", "[7,8,9]"];
        assert_eq!(result, expected);
    }
}
