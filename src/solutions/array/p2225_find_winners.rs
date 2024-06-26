use crate::solutions::Solution;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut losses: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
        matches.iter().for_each(|m| {
            let (winner, looser) = (m[0], m[1]);
            losses.entry(winner).or_insert(0);
            *losses.entry(looser).or_insert(0) += 1;
        });
        let mut no_losses = vec![];
        let mut one_losses = vec![];
        losses.iter().for_each(|(k, v)| {
            if *v == 0 {
                no_losses.push(*k);
            } else if *v == 1 {
                one_losses.push(*k);
            }
        });
        no_losses.sort_unstable();
        one_losses.sort_unstable();
        vec![no_losses, one_losses]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_winners() {
        let matches = vec![
            vec![1, 3],
            vec![2, 3],
            vec![3, 6],
            vec![5, 6],
            vec![5, 7],
            vec![4, 5],
            vec![4, 8],
            vec![4, 9],
            vec![10, 4],
            vec![10, 9],
        ];
        let res = vec![vec![1, 2, 10], vec![4, 5, 7, 8]];
        assert_eq!(Solution::find_winners(matches), res);
    }

    #[test]
    fn test_find_winners_2() {
        let matches = vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]];
        let res = vec![vec![1, 2, 5, 6], vec![]];
        assert_eq!(Solution::find_winners(matches), res);
    }
}
