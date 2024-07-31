use crate::solutions::Solution;

impl Solution {
    pub fn get_encrypted_string(mut s: String, k: i32) -> String {
        let chars = s.clone().into_bytes();
        let n = chars.len();
        let k = k as usize % n;
        s.clear();
        for i in 0..n {
            s.push(chars[(i + k) % n] as char);
        }
        s
    }

    pub fn get_encrypted_string_2(mut s: String, k: i32) -> String {
        s.split_off(k as usize % s.len()) + &s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_encrypted_string() {
        let s = "dart".to_string();
        let k = 3;
        let res = "tdar".to_string(); // cspell: disable-line
        assert_eq!(Solution::get_encrypted_string(s, k), res);
    }

    #[test]
    fn test_get_encrypted_string_2() {
        let s = "aaa".to_string();
        let k = 1;
        let res = "aaa".to_string();
        assert_eq!(Solution::get_encrypted_string(s, k), res);
    }

    #[test]
    fn test_get_encrypted_string_3() {
        let s = "abc".to_string();
        let k = 55;
        let res = "bca".to_string();
        assert_eq!(Solution::get_encrypted_string(s, k), res);
    }
}
