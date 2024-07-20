use crate::solutions::Solution;

impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        row_sum
            .iter_mut()
            .map(|row| {
                col_sum
                    .iter_mut()
                    .map(|col| {
                        let min_val = std::cmp::min(*row, *col);
                        *row -= min_val;
                        *col -= min_val;
                        min_val
                    })
                    .collect()
            })
            .collect()
    }

    pub fn restore_matrix_2(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; col_sum.len()]; row_sum.len()];

        row_sum.iter_mut().enumerate().for_each(|(i, row)| {
            col_sum.iter_mut().enumerate().for_each(|(j, col)| {
                let min_val = std::cmp::min(*row, *col);
                result[i][j] = min_val;
                *row -= min_val;
                *col -= min_val;
            });
        });

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restore_matrix() {
        let row_sum = vec![3, 8];
        let col_sum = vec![4, 7];
        let result = Solution::restore_matrix(row_sum.clone(), col_sum.clone());
        let result_row_sum = result
            .iter()
            .map(|row| row.iter().sum::<i32>())
            .collect::<Vec<i32>>();
        let result_col_sum = (0..result[0].len())
            .map(|j| result.iter().map(|row| row[j]).sum::<i32>())
            .collect::<Vec<i32>>();
        assert_eq!(result_row_sum, row_sum);
        assert_eq!(result_col_sum, col_sum);
    }

    #[test]
    fn test_restore_matrix_2() {
        let row_sum = vec![5, 7, 10];
        let col_sum = vec![8, 6, 8];
        let result = Solution::restore_matrix(row_sum.clone(), col_sum.clone());
        let result_row_sum = result
            .iter()
            .map(|row| row.iter().sum::<i32>())
            .collect::<Vec<i32>>();
        let result_col_sum = (0..result[0].len())
            .map(|j| result.iter().map(|row| row[j]).sum::<i32>())
            .collect::<Vec<i32>>();
        assert_eq!(result_row_sum, row_sum);
        assert_eq!(result_col_sum, col_sum);
    }

    #[test]
    fn test_restore_matrix_3() {
        let row_sum = vec![14, 9];
        let col_sum = vec![6, 9, 8];
        let result = Solution::restore_matrix(row_sum.clone(), col_sum.clone());
        let result_row_sum = result
            .iter()
            .map(|row| row.iter().sum::<i32>())
            .collect::<Vec<i32>>();
        let result_col_sum = (0..result[0].len())
            .map(|j| result.iter().map(|row| row[j]).sum::<i32>())
            .collect::<Vec<i32>>();
        assert_eq!(result_row_sum, row_sum);
        assert_eq!(result_col_sum, col_sum);
    }
}
