use crate::solutions::Solution;

impl Solution {
    pub fn min_swaps_2(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut zero_counts: Vec<(usize, usize)> = vec![(0, 0); n];
        for (i, row) in grid.iter().enumerate() {
            zero_counts[i] = (i, row.iter().rev().take_while(|&x| *x == 0).count());
        }
        let mut result = 0;
        for target in (1..n).rev() {
            let mut found = false;
            for (i, zero_count) in zero_counts.iter_mut().enumerate() {
                if zero_count.1 >= target {
                    let index = n - target - 1;
                    result += zero_count.0 - index;
                    zero_counts.remove(i);
                    found = true;
                    break;
                } else {
                    zero_count.0 += 1;
                }
            }
            if !found {
                return -1;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_swaps() {
        assert_eq!(
            Solution::min_swaps_2(vec![vec![0, 0, 1], vec![1, 1, 0], vec![1, 0, 0]]),
            3
        );
    }

    #[test]
    fn test_min_swaps_2() {
        assert_eq!(
            Solution::min_swaps_2(vec![
                vec![0, 1, 1, 0],
                vec![0, 1, 1, 0],
                vec![0, 1, 1, 0],
                vec![0, 1, 1, 0]
            ]),
            -1
        );
    }

    #[test]
    fn test_min_swaps_3() {
        assert_eq!(
            Solution::min_swaps_2(vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 1]]),
            0
        );
    }

    #[test]
    fn test_min_swaps_4() {
        assert_eq!(
            Solution::min_swaps_2(vec![
                vec![0, 1, 1, 0],
                vec![1, 1, 1, 0],
                vec![1, 1, 1, 0],
                vec![1, 0, 0, 0]
            ]),
            -1
        );
    }
}
