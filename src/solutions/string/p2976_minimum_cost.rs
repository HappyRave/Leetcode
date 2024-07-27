use crate::solutions::Solution;

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        let no_path = i64::MAX / 2;
        let char_a = 'a' as usize;
        let mut path = vec![vec![no_path; 26]; 26];

        cost.iter().enumerate().for_each(|(i, &cost)| {
            let o = original[i] as usize - char_a;
            let c = changed[i] as usize - char_a;
            path[o][c] = path[o][c].min(cost as i64);
        });

        (0..26).for_each(|k| {
            path[k][k] = 0;
            (0..26).for_each(|i| {
                (0..26).for_each(|j| {
                    path[i][j] = path[i][j].min(path[i][k] + path[k][j]);
                });
            });
        });

        source
            .chars()
            .zip(target.chars())
            .try_fold(0, |acc, (s, t)| {
                let s = s as usize - char_a;
                let t = t as usize - char_a;
                if path[s][t] == no_path {
                    return None;
                }
                Some(acc + path[s][t])
            })
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::ArrayStringExt;

    use super::*;

    #[test]
    fn test_minimum_cost() {
        let source = "abcd".to_string();
        let target = "acbe".to_string(); //cspell:disable-line
        let original = "[\"a\",\"b\",\"c\",\"c\",\"e\",\"d\"]".to_char_vec();
        let changed = "[\"b\",\"c\",\"b\",\"e\",\"b\",\"e\"]".to_char_vec();
        let cost = vec![2, 5, 5, 1, 2, 20];
        let res = 28;
        assert_eq!(
            Solution::minimum_cost(source, target, original, changed, cost),
            res
        );
    }

    #[test]
    fn test_minimum_cost_2() {
        let source = "aaaa".to_string();
        let target = "bbbb".to_string();
        let original = "[\"a\",\"c\"]".to_char_vec();
        let changed = "[\"c\",\"b\"]".to_char_vec();
        let cost = vec![1, 2];
        let res = 12;
        assert_eq!(
            Solution::minimum_cost(source, target, original, changed, cost),
            res
        );
    }

    #[test]
    fn test_minimum_cost_3() {
        let source = "abcd".to_string();
        let target = "abce".to_string(); //cspell:disable-line
        let original = "[\"a\"]".to_char_vec();
        let changed = "[\"e\"]".to_char_vec();
        let cost = vec![1000];
        let res = -1;
        assert_eq!(
            Solution::minimum_cost(source, target, original, changed, cost),
            res
        );
    }
}
