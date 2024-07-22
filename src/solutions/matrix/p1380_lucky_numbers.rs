use crate::solutions::Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let row_min = matrix
            .iter()
            .map(|row| row.iter().min().unwrap())
            .collect::<Vec<_>>();
        let col_max = (0..matrix[0].len())
            .map(|i| matrix.iter().map(|row| row[i]).max().unwrap())
            .collect::<std::collections::HashSet<_>>();
        row_min
            .iter()
            .filter(|&x| col_max.contains(x))
            .map(|&x| *x)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lucky_numbers() {
        assert_eq!(
            Solution::lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]]),
            vec![15]
        );
    }

    #[test]
    fn lucky_numbers_2() {
        assert_eq!(
            Solution::lucky_numbers(vec![
                vec![1, 10, 4, 2],
                vec![9, 3, 8, 7],
                vec![15, 16, 17, 12]
            ]),
            vec![12]
        );
    }

    #[test]
    fn lucky_numbers_3() {
        assert_eq!(
            Solution::lucky_numbers(vec![vec![7, 8], vec![1, 2]]),
            vec![7]
        );
    }
}
