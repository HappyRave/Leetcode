use crate::solutions::Solution;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let obstacles = obstacles
            .into_iter()
            .map(|v| (v[0], v[1]))
            .collect::<std::collections::HashSet<_>>();
        let mut x = 0;
        let mut y = 0;
        let mut direction = 0;
        let mut res = 0;
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for command in commands {
            if command == -2 {
                direction = (direction + 3) % 4;
            } else if command == -1 {
                direction = (direction + 1) % 4;
            } else {
                for _ in 0..command {
                    let next_x = x + directions[direction as usize].0;
                    let next_y = y + directions[direction as usize].1;
                    if obstacles.contains(&(next_x, next_y)) {
                        break;
                    }
                    x = next_x;
                    y = next_y;
                    res = res.max(x * x + y * y);
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_sim() {
        let commands = vec![4, -1, 3];
        let obstacles = vec![];
        let res = 25;
        assert_eq!(Solution::robot_sim(commands, obstacles), res);
    }

    #[test]
    fn test_robot_sim_2() {
        let commands = vec![4, -1, 4, -2, 4];
        let obstacles = vec![vec![2, 4]];
        let res = 65;
        assert_eq!(Solution::robot_sim(commands, obstacles), res);
    }
}
