use std::{cmp::Reverse, collections::BinaryHeap};

use crate::solutions::Solution;

impl Solution {
    pub fn min_groups(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by_key(|x| x[0]);
        let mut groups = BinaryHeap::<Reverse<i32>>::new();
        let mut max_groups = 0;
        for interval in intervals {
            if let Some(&Reverse(min_end)) = groups.peek() {
                if min_end < interval[0] {
                    groups.pop();
                }
            }
            groups.push(Reverse(interval[1]));
            max_groups = max_groups.max(groups.len());
        }
        max_groups as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::ArrayStringExt;

    use super::*;

    #[test]
    fn test_min_groups() {
        let intervals = "[[5,10],[6,8],[1,5],[2,3],[1,10]]".to_matrix();
        let res = 3;
        assert_eq!(Solution::min_groups(intervals), res);
    }

    #[test]
    fn test_min_groups_2() {
        let intervals = "[[1,3],[5,6],[8,10],[11,13]]".to_matrix();
        let res = 1;
        assert_eq!(Solution::min_groups(intervals), res);
    }
}
