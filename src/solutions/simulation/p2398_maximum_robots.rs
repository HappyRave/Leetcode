use crate::solutions::Solution;

impl Solution {
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        let n = charge_times.len();
        let mut prefix_sum: Vec<i64> = vec![0; running_costs.len() + 1];
        for i in 0..running_costs.len() {
            prefix_sum[i + 1] = prefix_sum[i] + running_costs[i] as i64;
        }

        let mut max_charge_time_deque = std::collections::VecDeque::<usize>::from(vec![0]);
        let (mut left, mut right, mut max_robots) = (0, 1, 0);

        while left < right && right <= n {
            let k = right - left;
            let total_running_cost: i64 = prefix_sum[right] - prefix_sum[left];
            let max_charge_time = charge_times[max_charge_time_deque[0]] as i64;

            // Check if the current window is within the budget
            if max_charge_time + total_running_cost * k as i64 <= budget {
                max_robots = max_robots.max((k) as i32);

                // Expand the window
                while let Some(&prev) = max_charge_time_deque.back() {
                    if right < n && charge_times[right] >= charge_times[prev] {
                        max_charge_time_deque.pop_back();
                    } else {
                        break;
                    }
                }
                max_charge_time_deque.push_back(right);
                right += 1;
            } else {
                left += 1;
                while let Some(&prev) = max_charge_time_deque.front() {
                    if prev < left {
                        max_charge_time_deque.pop_front();
                    } else {
                        break;
                    }
                }
                if left == right && right < n {
                    while let Some(&prev) = max_charge_time_deque.back() {
                        if right < n && charge_times[right] >= charge_times[prev] {
                            max_charge_time_deque.pop_back();
                        } else {
                            break;
                        }
                    }
                    max_charge_time_deque.push_back(right);
                    right += 1;
                }
            }
        }

        max_robots
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_robots() {
        let charge_times = vec![3, 6, 1, 3, 4];
        let running_costs = vec![2, 1, 3, 4, 5];
        let budget = 25;
        assert_eq!(
            Solution::maximum_robots(charge_times, running_costs, budget),
            3
        );
    }

    #[test]
    fn test_maximum_robots_2() {
        let charge_times = vec![11, 12, 19];
        let running_costs = vec![10, 8, 7];
        let budget = 19;
        assert_eq!(
            Solution::maximum_robots(charge_times, running_costs, budget),
            0
        );
    }

    #[test]
    fn test_maximum_robots_3() {
        let charge_times = vec![11, 12, 74, 67, 37, 87, 42, 34, 18, 90, 36, 28, 34, 20];
        let running_costs = vec![18, 98, 2, 84, 7, 57, 54, 65, 59, 91, 7, 23, 94, 20];
        let budget = 937;
        assert_eq!(
            Solution::maximum_robots(charge_times, running_costs, budget),
            4
        );
    }

    #[test]
    fn test_maximum_robots_4() {
        let charge_times = vec![
            19, 63, 21, 8, 5, 46, 56, 45, 54, 30, 92, 63, 31, 71, 87, 94, 67, 8, 19, 89, 79, 25,
        ];
        let running_costs = vec![
            91, 92, 39, 89, 62, 81, 33, 99, 28, 99, 86, 19, 5, 6, 19, 94, 65, 86, 17, 10, 8, 42,
        ];
        let budget = 85;
        assert_eq!(
            Solution::maximum_robots(charge_times, running_costs, budget),
            1
        );
    }

    #[test]
    fn test_maximum_robots_5() {
        let charge_times = (0..=100000).rev().collect::<Vec<i32>>();
        let mut running_costs = vec![1; 40000];
        running_costs.extend(vec![100000; 60001]);
        let budget = 100000000000;
        assert_eq!(
            Solution::maximum_robots(charge_times, running_costs, budget),
            40024
        );
    }
}
