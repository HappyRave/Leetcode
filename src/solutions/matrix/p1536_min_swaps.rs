use crate::solutions::Solution;

impl Solution {
    pub fn min_swaps_2(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut zero_counts: Vec<(usize, usize)> = vec![(0, 0); n];
        for (i, row) in grid.iter().enumerate() {
            let mut count = 0;
            for cell in row {
                if *cell == 0 {
                    count += 1;
                } else {
                    count = 0;
                }
            }
            zero_counts[i] = (i, count);
        }
        println!("{:?}", zero_counts);
        let mut result = 0;
        for row_count in (1..n).rev() {
            let mut found = false;
            println!("row_count: {}", row_count);
            for (i, zero_count) in zero_counts.iter_mut().enumerate() {
                zero_count.0 += 1;
                if zero_count.1 >= row_count {
                    println!("found {} at {}", row_count, i);
                    result += n - row_count - 1 + zero_count.0;
                    zero_counts.remove(i);
                    found = true;
                    break;
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
}
