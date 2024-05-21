use super::Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];
        Self::backtrack(&nums, 0, &mut path, &mut res);
        res
    }

    fn backtrack(nums: &Vec<i32>, start: usize, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        res.push(path.clone());
        for i in start..nums.len() {
            path.push(nums[i]);
            Self::backtrack(nums, i + 1, path, res);
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3];
        let res = [
            vec![3],
            vec![1],
            vec![2],
            vec![1, 2, 3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2],
            vec![],
        ];
        let solution = Solution::subsets(nums);
        assert_eq!(solution.len(), res.len());
        for i in solution {
            assert!(res.contains(&i));
        }
    }

    #[test]
    fn test_2() {
        let nums = vec![0];
        let res = [vec![], vec![0]];
        let solution = Solution::subsets(nums);
        assert_eq!(solution.len(), res.len());
        for i in solution {
            assert!(res.contains(&i));
        }
    }
}
