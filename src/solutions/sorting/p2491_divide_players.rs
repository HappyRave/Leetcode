use crate::solutions::Solution;

impl Solution {
    pub fn divide_players(mut skill: Vec<i32>) -> i64 {
        let n = skill.len();
        let sum = skill.iter().map(|&x| x as i64).sum::<i64>();

        if sum % (n as i64 / 2) != 0 {
            return -1;
        }

        skill.sort_unstable();
        let target = skill[0] + skill[n - 1];
        let mut result = 0i64;

        for i in 0..n / 2 {
            if skill[i] + skill[n - i - 1] != target {
                return -1;
            }
            result += skill[n - i - 1] as i64 * skill[i] as i64;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_players() {
        let skill = vec![3, 2, 5, 1, 3, 4];
        let result = 22;
        assert_eq!(Solution::divide_players(skill), result);
    }

    #[test]
    fn test_divide_players_2() {
        let skill = vec![3, 4];
        let result = 12;
        assert_eq!(Solution::divide_players(skill), result);
    }

    #[test]
    fn test_divide_players_3() {
        let skill = vec![1, 1, 2, 3];
        let result = -1;
        assert_eq!(Solution::divide_players(skill), result);
    }
}
