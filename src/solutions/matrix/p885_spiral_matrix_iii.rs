use crate::solutions::Solution;

impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let mut cells = rows * cols;
        let mut result = vec![];
        let mut steps = 1;
        let mut r = r_start;
        let mut c = c_start;
        let mut direction = 0;
        while cells > 0 {
            for _ in 0..steps {
                if r >= 0 && r < rows && c >= 0 && c < cols {
                    result.push(vec![r, c]);
                    cells -= 1;
                }
                match direction {
                    0 => c += 1,
                    1 => r += 1,
                    2 => c -= 1,
                    3 => r -= 1,
                    _ => (),
                }
            }
            direction = (direction + 1) % 4;
            if direction % 2 == 0 {
                steps += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::ArrayStringExt;

    use super::*;

    #[test]
    fn p885_spiral_matrix_iii_t1() {
        let solution = "[[0,0],[0,1],[0,2],[0,3]]".to_matrix();
        let answer = Solution::spiral_matrix_iii(1, 4, 0, 0);
        assert_eq!(answer, solution);
    }

    #[test]
    fn p885_spiral_matrix_iii_t2() {
        let solution = "[[1,4],[1,5],[2,5],[2,4],[2,3],[1,3],[0,3],[0,4],[0,5],[3,5],[3,4],[3,3],[3,2],[2,2],[1,2],[0,2],[4,5],[4,4],[4,3],[4,2],[4,1],[3,1],[2,1],[1,1],[0,1],[4,0],[3,0],[2,0],[1,0],[0,0]]".to_matrix();
        let answer = Solution::spiral_matrix_iii(5, 6, 1, 4);
        assert_eq!(answer, solution);
    }
}
