use crate::solutions::Solution;

impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut wait_time: i64 = 0;
        let mut current_time: i64 = 0;
        for customer in &customers {
            let arrival_time = customer[0] as i64;
            let time_to_cook = customer[1] as i64;
            let start_time = if current_time > arrival_time {
                current_time
            } else {
                arrival_time
            };
            wait_time += start_time + time_to_cook - arrival_time;
            current_time = start_time + time_to_cook;
        }
        wait_time as f64 / customers.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average_waiting_time() {
        let customers = vec![vec![1, 2], vec![2, 5], vec![4, 3]];
        assert_eq!(Solution::average_waiting_time(customers), 5.0);
    }

    #[test]
    fn test_average_waiting_time_2() {
        let customers = vec![vec![5, 2], vec![5, 4], vec![10, 3], vec![20, 1]];
        assert_eq!(Solution::average_waiting_time(customers), 3.25);
    }
}
