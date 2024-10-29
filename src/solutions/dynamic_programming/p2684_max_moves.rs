use crate::solutions::Solution;

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let directions = [(-1, 1), (0, 1), (1, 1)];
        let (n, m) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![0; m]; n];
        let mut max_moves = 0;
        for j in (0..m).rev() {
            for i in (0..n).rev() {
                if j < m - 1 {
                    let mut max_move = 0;
                    let mut path = false;
                    for &(di, dj) in &directions {
                        let ni = i as isize + di;
                        let nj = j as isize + dj;
                        if ni >= 0
                            && ni < n as isize
                            && nj < m as isize
                            && grid[i][j] < grid[ni as usize][nj as usize]
                        {
                            path = true;
                            max_move = max_move.max(dp[ni as usize][nj as usize]);
                        }
                    }

                    dp[i][j] = if path { max_move + 1 } else { 0 };
                }

                if j == 0 {
                    max_moves = max_moves.max(dp[i][j]);
                }
            }
        }
        max_moves
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::ArrayStringExt;

    use super::*;

    #[test]
    fn test_max_moves() {
        let grid = "[[2,4,3,5],[5,4,9,3],[3,4,2,11],[10,9,13,15]]".to_matrix();
        assert_eq!(Solution::max_moves(grid), 3);
    }

    #[test]
    fn test_max_moves_2() {
        let grid = "[[3,2,4],[2,1,9],[1,1,7]]".to_matrix();
        assert_eq!(Solution::max_moves(grid), 0);
    }

    #[test]
    fn test_max_moves_3() {
        let grid = "[[65,200,263,220,91,183,2,187,175,61,225,120,39],[111,242,294,31,241,90,145,25,262,214,145,71,294],[152,25,240,69,279,238,222,9,137,277,8,143,143],[189,31,86,250,20,63,188,209,75,22,127,272,110],[122,94,298,25,90,169,68,3,208,274,202,135,275],[205,20,171,90,70,272,280,138,142,151,80,122,130],[284,272,271,269,265,134,185,243,247,50,283,20,232],[266,236,265,234,249,62,98,130,122,226,285,168,204],[231,24,256,101,142,28,268,82,111,63,115,13,144],[277,277,31,144,49,132,28,138,133,29,286,45,93],[163,96,25,9,3,159,148,59,25,81,233,127,12],[127,38,31,209,300,256,15,43,74,64,73,141,200]]".to_matrix();
        assert_eq!(Solution::max_moves(grid), 3);
    }
}
