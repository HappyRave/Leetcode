use crate::solutions::Solution;

impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let mut robots: Vec<usize> = (0..positions.len()).collect();
        robots.sort_unstable_by_key(|&i| positions[i]);

        let mut survive = Vec::<(usize, i32)>::new();
        let mut stack = Vec::<(usize, i32)>::new();
        for &i in robots.iter() {
            if directions.as_bytes()[i] == b'L' {
                let mut hp = healths[i];
                while hp != 0 && !stack.is_empty() {
                    let last = stack.len() - 1;
                    match stack[last].1.cmp(&hp) {
                        std::cmp::Ordering::Less => {
                            stack.pop();
                            hp -= 1;
                        }
                        std::cmp::Ordering::Greater => {
                            stack[last].1 -= 1;
                            hp = 0;
                        }
                        std::cmp::Ordering::Equal => {
                            stack.pop();
                            hp = 0;
                        }
                    }
                }
                if hp != 0 {
                    survive.push((i, hp));
                }
            } else {
                stack.push((i, healths[i]));
            }
        }
        survive.append(&mut stack);
        survive.sort_unstable();
        survive.iter().map(|&(_, hp)| hp).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_survived_robots_healths() {
        let positions = vec![5, 4, 3, 2, 1];
        let healths = vec![2, 17, 9, 15, 10];
        let directions = "RRRRR".to_string();
        let result = Solution::survived_robots_healths(positions, healths, directions);
        assert_eq!(result, vec![2, 17, 9, 15, 10]);
    }

    #[test]
    fn test_survived_robots_healths_2() {
        let positions = vec![3, 5, 2, 6];
        let healths = vec![10, 10, 15, 12];
        let directions = "RLRL".to_string();
        let result = Solution::survived_robots_healths(positions, healths, directions);
        assert_eq!(result, vec![14]);
    }

    #[test]
    fn test_survived_robots_healths_3() {
        let positions = vec![1, 2, 5, 6];
        let healths = vec![10, 10, 11, 11];
        let directions = "RLRL".to_string();
        let result = Solution::survived_robots_healths(positions, healths, directions);
        assert_eq!(result, vec![]);
    }
}
