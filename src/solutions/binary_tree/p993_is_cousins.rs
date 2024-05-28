use std::cell::RefCell;
use std::rc::Rc;

use crate::solutions::Solution;

use super::TreeNode;
impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            parent: Option<Rc<RefCell<TreeNode>>>,
            depth: i32,
            val: i32,
        ) -> (Option<Rc<RefCell<TreeNode>>>, i32) {
            if let Some(node) = root {
                let node_ref = node.borrow();
                if node_ref.val == val {
                    return (parent, depth);
                }
                let (left_parent, left_depth) =
                    dfs(node_ref.left.clone(), Some(node.clone()), depth + 1, val);
                if left_parent.is_some() {
                    return (left_parent, left_depth);
                }
                let (right_parent, right_depth) =
                    dfs(node_ref.right.clone(), Some(node.clone()), depth + 1, val);
                if right_parent.is_some() {
                    return (right_parent, right_depth);
                }
            }
            (None, 0)
        }

        let (x_parent, x_depth) = dfs(root.clone(), None, 0, x);
        let (y_parent, y_depth) = dfs(root.clone(), None, 0, y);
        x_parent != y_parent && x_depth == y_depth
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::binary_tree::BinaryTreeExt;

    use super::*;

    #[test]
    fn test_is_cousins() {
        let tree = vec![Some(1), Some(2), Some(3), Some(4)];
        let root = tree.into_tree();
        assert!(!Solution::is_cousins(root, 4, 3));
    }

    #[test]
    fn test_is_cousins_2() {
        let tree = vec![Some(1), Some(2), Some(3), None, Some(4), None, Some(5)];
        let root = tree.into_tree();
        assert!(Solution::is_cousins(root, 5, 4));
    }

    #[test]
    fn test_is_cousins_3() {
        let tree = vec![Some(1), Some(2), Some(3), None, Some(4)];
        let root = tree.into_tree();
        assert!(!Solution::is_cousins(root, 2, 3));
    }
}
