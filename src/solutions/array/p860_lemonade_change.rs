use crate::solutions::Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        if bills[0] != 5 {
            return false;
        }
        let mut five = 1;
        let mut ten = 0;
        for bill in bills.iter().skip(1) {
            match bill {
                5 => five += 1,
                10 => {
                    if five == 0 {
                        return false;
                    }
                    five -= 1;
                    ten += 1;
                }
                20 => {
                    if ten > 0 && five > 0 {
                        ten -= 1;
                        five -= 1;
                    } else if five >= 3 {
                        five -= 3;
                    } else {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lemonade_change() {
        let bills = vec![5, 5, 5, 10, 20];
        let result = true;
        assert_eq!(Solution::lemonade_change(bills), result);
    }

    #[test]
    fn test_lemonade_change_2() {
        let bills = vec![5, 5, 10];
        let result = true;
        assert_eq!(Solution::lemonade_change(bills), result);
    }

    #[test]
    fn test_lemonade_change_3() {
        let bills = vec![10, 10];
        let result = false;
        assert_eq!(Solution::lemonade_change(bills), result);
    }

    #[test]
    fn test_lemonade_change_4() {
        let bills = vec![5, 5, 10, 10, 20];
        let result = false;
        assert_eq!(Solution::lemonade_change(bills), result);
    }
}
