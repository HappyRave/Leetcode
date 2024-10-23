use std::cell::RefCell;
use std::rc::Rc;

use crate::solutions::Solution;

use super::TreeNode;
impl Solution {
    pub fn replace_value_in_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root?;

        let mut level_sum = 0;
        let mut queue = vec![(root.clone(), 0)];

        while !queue.is_empty() {
            let mut next_queue = Vec::new();
            let mut new_level_sum = 0;

            for (node, sibling_sum) in queue {
                node.borrow_mut().val = level_sum - sibling_sum;

                let mut sibling_sum = 0;
                let mut non_leaf_children = vec![];

                if let Some(left) = node.borrow().left.clone() {
                    sibling_sum += left.borrow().val;
                    non_leaf_children.push(left);
                }

                if let Some(right) = node.borrow().right.clone() {
                    sibling_sum += right.borrow().val;
                    non_leaf_children.push(right);
                }

                new_level_sum += sibling_sum;
                next_queue.extend(
                    non_leaf_children
                        .into_iter()
                        .map(|node| (node, sibling_sum)),
                );
            }

            level_sum = new_level_sum;
            queue = next_queue;
        }

        Some(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_value_in_tree() {
        let root = TreeNode::from_string("5,4,9,1,10,null,7");
        let ans = TreeNode::from_string("0,0,0,7,7,null,11");
        let result = Solution::replace_value_in_tree(root);
        assert_eq!(result, ans);
    }

    #[test]
    fn test_replace_value_in_tree_2() {
        let root = TreeNode::from_string("3,1,2");
        let ans = TreeNode::from_string("0,0,0");
        let result = Solution::replace_value_in_tree(root);
        assert_eq!(result, ans);
    }
}
