use crate::solutions::Solution;

impl Solution {
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        seats.sort_unstable();
        students.sort_unstable();
        seats
            .iter()
            .zip(students.iter())
            .map(|(&a, &b)| (a - b).abs())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_moves_to_seat() {
        let seats = vec![3, 1, 5];
        let students = vec![2, 7, 4];
        let result = 4;
        assert_eq!(Solution::min_moves_to_seat(seats, students), result);
    }

    #[test]
    fn test_min_moves_to_seat_2() {
        let seats = vec![4, 1, 5, 9];
        let students = vec![1, 3, 2, 6];
        let result = 7;
        assert_eq!(Solution::min_moves_to_seat(seats, students), result);
    }

    #[test]
    fn test_min_moves_to_seat_3() {
        let seats = vec![2, 2, 6, 6];
        let students = vec![1, 3, 2, 6];
        let result = 4;
        assert_eq!(Solution::min_moves_to_seat(seats, students), result);
    }
}
