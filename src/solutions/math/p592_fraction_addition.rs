use crate::solutions::Solution;

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                (a, b) = (b, a % b);
            }
            a
        }

        let expr = expression.as_bytes();

        let consume_sign = |i: usize| match expr[i] {
            b'+' => (1, i + 1),
            b'-' => (-1, i + 1),
            _ => (1, i),
        };

        let consume_num = |i: usize| {
            if i < expr.len() - 1 && expr[i + 1] == b'0' {
                (10, i + 2)
            } else {
                ((expr[i] - b'0') as i32, i + 1)
            }
        };

        let mut i = 0;
        let mut whole_part = 0;
        let mut num_acc = 0i32;
        let den_acc: i32 = 5 * 7 * 8 * 9;
        while i < expr.len() {
            let (sign, num, den);
            (sign, i) = consume_sign(i);
            (num, i) = consume_num(i);
            i += 1;
            (den, i) = consume_num(i);
            if num == 0 {
                continue;
            }
            let num = sign * num;
            whole_part += num / den;
            num_acc += den_acc / den * (num % den)
        }
        let common_den = gcd(den_acc, num_acc.abs());
        let (acc_num, den_acc) = (
            num_acc / common_den + den_acc / common_den * whole_part,
            den_acc / common_den,
        );
        acc_num.to_string() + "/" + &den_acc.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fraction_addition() {
        assert_eq!(
            Solution::fraction_addition("1/3-1/2".to_string()),
            "-1/6".to_string()
        );
    }

    #[test]
    fn test_fraction_addition_2() {
        assert_eq!(
            Solution::fraction_addition("-1/2+1/2".to_string()),
            "0/1".to_string()
        );
    }

    #[test]
    fn test_fraction_addition_3() {
        assert_eq!(
            Solution::fraction_addition("-1/2+1/2+1/3".to_string()),
            "1/3".to_string()
        );
    }
}
