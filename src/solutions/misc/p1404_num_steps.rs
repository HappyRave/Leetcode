use crate::solutions::Solution;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut s = s.chars().collect::<Vec<char>>();
        let mut steps = 0;
        while s.len() > 1 {
            if s[s.len() - 1] == '0' {
                s.pop();
            } else {
                let mut carry = true;
                for i in (0..s.len()).rev() {
                    if s[i] == '1' {
                        if carry {
                            s[i] = '0';
                        } else {
                            s[i] = '1';
                            break;
                        }
                    } else if carry {
                        s[i] = '1';
                        carry = false;
                    } else {
                        break;
                    }
                }
                if carry {
                    s.insert(0, '1');
                }
            }
            steps += 1;
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_steps() {
        assert_eq!(Solution::num_steps("1101".to_string()), 6);
    }

    #[test]
    fn test_num_steps_2() {
        assert_eq!(Solution::num_steps("10".to_string()), 1);
    }

    #[test]
    fn test_num_steps_3() {
        assert_eq!(Solution::num_steps("1".to_string()), 0);
    }

    #[test]
    fn test_num_steps_4() {
        assert_eq!(
            Solution::num_steps(
                "100100001010010101101000111100100111101111000111111010010001100001000100011001"
                    .to_string()
            ),
            120
        );
    }
}
