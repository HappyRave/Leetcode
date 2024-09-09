use crate::solutions::Solution;

use super::ListNode;

impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![-1; n as usize]; m as usize];
        let mut head = head;
        let mut row = 0;
        let mut col = 0;
        let mut direction = 0;
        let mut top = 0;
        let mut bottom = m - 1;
        let mut left = 0;
        let mut right = n - 1;

        while let Some(node) = head {
            matrix[row as usize][col as usize] = node.val;
            head = node.next;
            match direction {
                0 => {
                    if col == right {
                        direction = 1;
                        top += 1;
                        row += 1;
                    } else {
                        col += 1;
                    }
                }
                1 => {
                    if row == bottom {
                        direction = 2;
                        right -= 1;
                        col -= 1;
                    } else {
                        row += 1;
                    }
                }
                2 => {
                    if col == left {
                        direction = 3;
                        bottom -= 1;
                        row -= 1;
                    } else {
                        col -= 1;
                    }
                }
                3 => {
                    if row == top {
                        direction = 0;
                        left += 1;
                        col += 1;
                    } else {
                        row -= 1;
                    }
                }
                _ => unreachable!(),
            }
        }

        matrix
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::{linked_lists::ListExt, ArrayStringExt};

    use super::*;

    #[test]
    fn test_spiral_matrix() {
        let m = 3;
        let n = 5;
        let head = vec![3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0].into_list();
        let result = "[[3,0,2,6,8],[5,0,-1,-1,1],[5,2,4,9,7]]".to_matrix();
        assert_eq!(Solution::spiral_matrix(m, n, head), result);
    }

    #[test]
    fn test_spiral_matrix_2() {
        let m = 1;
        let n = 4;
        let head = vec![0, 1, 2].into_list();
        let result = "[[0,1,2,-1]]".to_matrix();
        assert_eq!(Solution::spiral_matrix(m, n, head), result);
    }
}
