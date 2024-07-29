use crate::solutions::Solution;

impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 0..rating.len() - 1 {
            let (mut left_smaller, mut left_larger) = (0, 0);
            let (mut right_smaller, mut right_larger) = (0, 0);
            for j in 0..i {
                if rating[j] < rating[i] {
                    left_smaller += 1;
                } else {
                    left_larger += 1;
                }
            }
            for j in i + 1..rating.len() {
                if rating[j] < rating[i] {
                    right_smaller += 1;
                } else {
                    right_larger += 1;
                }
            }
            count += left_smaller * right_larger + left_larger * right_smaller
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1395_num_teams() {
        assert_eq!(Solution::num_teams(vec![2, 5, 3, 4, 1]), 3);
    }

    #[test]
    fn p1395_num_teams_2() {
        assert_eq!(Solution::num_teams(vec![2, 1, 3]), 0);
    }

    #[test]
    fn p1395_num_teams_3() {
        assert_eq!(Solution::num_teams(vec![1, 2, 3, 4]), 4);
    }
}
