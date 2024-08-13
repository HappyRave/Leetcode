use crate::solutions::Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn dfs(candidates: &[i32], target: i32, start: usize) -> Vec<Vec<i32>> {
            if target == 0 {
                return vec![vec![]];
            }
            if start >= candidates.len() {
                return vec![];
            }
            let mut result = vec![];
            for i in start..candidates.len() {
                if i > start && candidates[i] == candidates[i - 1] {
                    continue;
                }
                if candidates[i] > target {
                    break;
                }
                let mut sub_result = dfs(candidates, target - candidates[i], i + 1);
                for sub in sub_result.iter_mut() {
                    sub.insert(0, candidates[i]);
                }
                result.append(&mut sub_result);
            }
            result
        }
        let mut candidates = candidates;
        candidates.sort_unstable();
        dfs(&candidates, target, 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::ArrayStringExt;

    use super::*;

    #[test]
    fn test_1() {
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let target = 8;
        let result = "[[1,1,6],[1,2,5],[1,7],[2,6]]".to_matrix();
        assert_eq!(Solution::combination_sum2(candidates, target), result);
    }

    #[test]
    fn test_2() {
        let candidates = vec![2, 5, 2, 1, 2];
        let target = 5;
        let result = "[[1,2,2],[5]]".to_matrix();
        assert_eq!(Solution::combination_sum2(candidates, target), result);
    }
}
