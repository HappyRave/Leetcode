use crate::solutions::Solution;

impl Solution {
    pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
        let group_size = group_size as usize;
        let mut map = std::collections::HashMap::new();
        hand.sort();
        for card in &hand {
            *map.entry(card).or_insert(0) += 1;
        }

        for card in &hand {
            if map[&card] == 0 {
                continue;
            }

            for i in 0..group_size {
                if let Some(count) = map.get_mut(&(card + i as i32)) {
                    if *count == 0 {
                        return false;
                    }
                    *count -= 1;
                } else {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_n_straight_hand() {
        let hand = vec![1, 2, 3, 6, 2, 3, 4, 7, 8];
        let group_size = 3;
        let result = true;

        assert_eq!(Solution::is_n_straight_hand(hand, group_size), result);
    }

    #[test]
    fn test_is_n_straight_hand_2() {
        let hand = vec![1, 2, 3, 4, 5];
        let group_size = 4;
        let result = false;

        assert_eq!(Solution::is_n_straight_hand(hand, group_size), result);
    }

    #[test]
    fn testis_n_straight_hand_3() {
        let hand = vec![1, 2, 3, 4, 5];
        let group_size = 5;
        let result = true;

        assert_eq!(Solution::is_n_straight_hand(hand, group_size), result);
    }
}
