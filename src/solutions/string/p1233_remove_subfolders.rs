use crate::solutions::Solution;

impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        folder.sort_unstable();
        let mut res = vec![folder[0].clone()];

        for f in folder.iter().skip(1) {
            let last = res.last().unwrap().clone() + "/";
            if !f.starts_with(&last) {
                res.push(f.clone());
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_subfolders() {
        let folder = vec![
            "/a".to_string(),
            "/a/b".to_string(),
            "/c/d".to_string(),
            "/c/d/e".to_string(),
            "/c/f".to_string(),
        ];
        let result = vec!["/a".to_string(), "/c/d".to_string(), "/c/f".to_string()];
        assert_eq!(Solution::remove_subfolders(folder), result);
    }

    #[test]
    fn test_remove_subfolders_2() {
        let folder = vec!["/a".to_string(), "/a/b/c".to_string(), "/a/b/d".to_string()];
        let result = vec!["/a".to_string()];
        assert_eq!(Solution::remove_subfolders(folder), result);
    }

    #[test]
    fn test_remove_subfolders_3() {
        let folder = vec![
            "/a/b/c".to_string(),
            "/a/b/ca".to_string(),
            "/a/b/d".to_string(),
            "/a/b/da".to_string(),
        ];
        let result = vec![
            "/a/b/c".to_string(),
            "/a/b/ca".to_string(),
            "/a/b/d".to_string(),
            "/a/b/da".to_string(),
        ];
        assert_eq!(Solution::remove_subfolders(folder), result);
    }
}
