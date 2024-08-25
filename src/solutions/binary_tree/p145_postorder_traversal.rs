use std::cell::RefCell;
use std::rc::Rc;

use crate::solutions::Solution;

use super::TreeNode;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut stack = vec![root];

        while !stack.is_empty() {
            if let Some(Some(node)) = stack.pop() {
                result.insert(0, node.borrow().val);
                stack.push(node.borrow().left.clone());
                stack.push(node.borrow().right.clone());
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postorder_traversal() {
        let root = TreeNode::from_string("1,null,2,3");
        assert_eq!(Solution::postorder_traversal(root), vec![3, 2, 1]);
    }

    #[test]
    fn test_postorder_traversal_2() {
        let root = TreeNode::from_string("");
        assert_eq!(Solution::postorder_traversal(root), vec![]);
    }

    #[test]
    fn test_postorder_traversal_3() {
        let root = TreeNode::from_string("1");
        assert_eq!(Solution::postorder_traversal(root), vec![1]);
    }

    #[test]
    fn test_postorder_traversal_4() {
        let root = TreeNode::from_string("1,3,3,3,3,3");
        assert_eq!(Solution::postorder_traversal(root), vec![3, 3, 3, 3, 3, 1]);
    }
}
