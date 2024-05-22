use crate::solutions::Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn is_palindrome(s: &str) -> bool {
            let s = s.chars().collect::<Vec<char>>();
            let (mut i, mut j) = (0, s.len() - 1);
            while i < j {
                if s[i] != s[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }
            true
        }

        fn backtrack(s: &str, start: usize, path: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
            if start == s.len() {
                res.push(path.clone());
                return;
            }
            for i in start..s.len() {
                let sub = &s[start..=i];
                if is_palindrome(sub) {
                    path.push(sub.to_string());
                    backtrack(s, i + 1, path, res);
                    path.pop();
                }
            }
        }

        let mut res = vec![];
        let mut path = vec![];
        backtrack(&s, 0, &mut path, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "aab".to_string();
        let res: Vec<Vec<String>> = [vec!["a", "a", "b"], vec!["aa", "b"]]
            .iter()
            .map(|x| x.iter().map(|x| x.to_string()).collect())
            .collect();
        let solution = Solution::partition(s);
        assert_eq!(solution.len(), res.len());
        for i in solution {
            assert!(res.contains(&i));
        }
    }

    #[test]
    fn test_2() {
        let s = "a".to_string();
        let res: Vec<Vec<String>> = [vec!["a"]]
            .iter()
            .map(|x| x.iter().map(|x| x.to_string()).collect())
            .collect();
        let solution = Solution::partition(s);
        assert_eq!(solution.len(), res.len());
        for i in solution {
            assert!(res.contains(&i));
        }
    }
}
