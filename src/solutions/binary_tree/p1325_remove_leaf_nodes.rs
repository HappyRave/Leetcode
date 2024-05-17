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
    use crate::solutions::binary_tree::BinaryTreeExt;

    use super::*;

    #[test]
    fn test_1() {
        let tree = vec![Some(1), Some(2), Some(3), Some(2), None, Some(2), Some(4)];
        let root = tree.into_tree();
        let target = 2;
        let res = vec![Some(1), None, Some(3), None, Some(4)];
        assert_eq!(Solution::remove_leaf_nodes(root, target), res.into_tree());
    }

    #[test]
    fn test_2() {
        let tree = vec![Some(1), Some(3), Some(3), Some(3), Some(2)];
        let root = tree.into_tree();
        let target = 3;
        let res = vec![Some(1), Some(3), None, None, Some(2)];
        assert_eq!(Solution::remove_leaf_nodes(root, target), res.into_tree());
    }

    #[test]
    fn test_3() {
        let tree = vec![Some(1), Some(2), None, Some(2), None, Some(2)];
        let root = tree.into_tree();
        let target = 2;
        let res = vec![Some(1)];
        assert_eq!(Solution::remove_leaf_nodes(root, target), res.into_tree());
    }
}
