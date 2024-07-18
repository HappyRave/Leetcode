use std::cell::RefCell;
use std::rc::Rc;

use crate::solutions::Solution;

use super::TreeNode;

impl Solution {
    pub fn remove_leaf_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(
            root: &mut Option<Rc<RefCell<TreeNode>>>,
            target: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node) = root {
                let mut node = node.borrow_mut();
                node.left = helper(&mut node.left, target);
                node.right = helper(&mut node.right, target);
                if node.left.is_none() && node.right.is_none() && node.val == target {
                    return None;
                }
            }
            root.clone()
        }
        helper(&mut root.clone(), target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_leaf_nodes() {
        let tree = TreeNode::from_string("1,2,3,2,null,2,4");
        assert_eq!(
            Solution::remove_leaf_nodes(tree, 2),
            TreeNode::from_string("1,null,3,null,4")
        );
    }

    #[test]
    fn test_remove_leaf_nodes_2() {
        let tree = TreeNode::from_string("1,3,3,3,2");
        assert_eq!(
            Solution::remove_leaf_nodes(tree, 3),
            TreeNode::from_string("1,3,null,null,2")
        );
    }

    #[test]
    fn test_remove_leaf_nodes_3() {
        let tree = TreeNode::from_string("1,2,null,2,null,2");
        assert_eq!(
            Solution::remove_leaf_nodes(tree, 2),
            TreeNode::from_string("1")
        );
    }
}
