use std::cell::RefCell;
use std::rc::Rc;

use crate::solutions::Solution;

use super::TreeNode;
impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        if root.is_none() {
            return -1;
        }

        let mut level_sum = vec![];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.unwrap());

        while !queue.is_empty() {
            let mut sum = 0_i64;
            let level_size = queue.len();
            for _ in 0..level_size {
                if let Some(node) = queue.pop_front() {
                    let node = node.borrow();
                    sum += node.val as i64;
                    if let Some(left) = node.left.clone() {
                        queue.push_back(left);
                    }
                    if let Some(right) = node.right.clone() {
                        queue.push_back(right);
                    }
                }
            }
            level_sum.push(sum);
        }

        if level_sum.len() < k as usize {
            return -1;
        }

        level_sum.sort_unstable();
        level_sum[level_sum.len() - k as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_largest_level_sum() {
        let root = TreeNode::from_string("5,8,9,2,1,3,7,4,6");
        let k = 2;
        assert_eq!(Solution::kth_largest_level_sum(root, k), 13);
    }

    #[test]
    fn test_kth_largest_level_sum_2() {
        let root = TreeNode::from_string("1,2,null,3");
        let k = 1;
        assert_eq!(Solution::kth_largest_level_sum(root, k), 3);
    }
}
