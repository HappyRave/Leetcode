use crate::solutions::Solution;

use super::ListNode;

impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut current = head;
        let mut critical_points = vec![];
        let mut prev: Option<Box<ListNode>> = None;
        let mut index = 0;
        while let Some(mut node) = current {
            let next = node.next.take();
            if let (Some(prev), Some(next)) = (&prev, &next) {
                if (prev.val > node.val && node.val < next.val)
                    || (prev.val < node.val && node.val > next.val)
                {
                    critical_points.push(index);
                }
            }
            index += 1;
            prev = Some(node);
            current = next;
        }
        if critical_points.len() < 2 {
            return vec![-1, -1];
        }
        let max = critical_points.last().unwrap() - critical_points.first().unwrap();
        let min = critical_points
            .windows(2)
            .map(|w| w[1] - w[0])
            .min()
            .unwrap();
        vec![min, max]
    }
}

#[cfg(test)]
mod tests {

    use crate::solutions::linked_lists::ListExt;

    use super::*;

    #[test]
    fn test_nodes_between_critical_points() {
        let head = vec![3, 1].into_list();
        let solution = vec![-1, -1];
        let result = Solution::nodes_between_critical_points(head);
        assert_eq!(result, solution);
    }

    #[test]
    fn test_nodes_between_critical_points_2() {
        let head = vec![5, 3, 1, 2, 5, 1, 2].into_list();
        let solution = vec![1, 3];
        let result = Solution::nodes_between_critical_points(head);
        assert_eq!(result, solution);
    }

    #[test]
    fn test_nodes_between_critical_points_3() {
        let head = vec![1, 3, 2, 2, 3, 2, 2, 2, 7].into_list();
        let solution = vec![3, 3];
        let result = Solution::nodes_between_critical_points(head);
        assert_eq!(result, solution);
    }
}
