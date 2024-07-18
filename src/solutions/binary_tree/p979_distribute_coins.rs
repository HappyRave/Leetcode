use std::cell::RefCell;
use std::rc::Rc;

use crate::solutions::Solution;

use super::TreeNode;
impl Solution {
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, moves: &mut i32) -> i32 {
            if let Some(node) = node {
                let node = node.borrow();
                let left = dfs(&node.left, moves);
                let right = dfs(&node.right, moves);
                *moves += left.abs() + right.abs();
                node.val + left + right - 1
            } else {
                0
            }
        }
        let mut moves = 0;
        dfs(&root, &mut moves);
        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distribute_coins() {
        let tree = TreeNode::from_string("3,0,0");
        assert_eq!(Solution::distribute_coins(tree), 2);
    }

    #[test]
    fn test_distribute_coins_2() {
        let tree = TreeNode::from_string("0,3,0");
        assert_eq!(Solution::distribute_coins(tree), 3);
    }
}
