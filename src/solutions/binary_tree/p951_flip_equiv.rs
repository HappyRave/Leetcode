use std::cell::RefCell;
use std::rc::Rc;

use crate::solutions::Solution;

use super::TreeNode;
impl Solution {
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn dfs(
            root1: Option<&Rc<RefCell<TreeNode>>>,
            root2: Option<&Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (root1, root2) {
                (None, None) => true,
                (Some(_), None) | (None, Some(_)) => false,
                (Some(node1), Some(node2)) => {
                    let node1 = node1.borrow();
                    let node2 = node2.borrow();
                    node1.val == node2.val
                        && ((dfs(node1.left.as_ref(), node2.left.as_ref())
                            && dfs(node1.right.as_ref(), node2.right.as_ref()))
                            || (dfs(node1.left.as_ref(), node2.right.as_ref())
                                && dfs(node1.right.as_ref(), node2.left.as_ref())))
                }
            }
        }

        dfs(root1.as_ref(), root2.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_equiv() {
        let root1 = TreeNode::from_string("1,2,3,4,5,6,null,null,null,7,8");
        let root2 = TreeNode::from_string("1,3,2,null,6,4,5,null,null,null,null,8,7");
        assert!(Solution::flip_equiv(root1, root2));
    }

    #[test]
    fn test_flip_equiv_2() {
        let root1 = TreeNode::from_string("");
        let root2 = TreeNode::from_string("");
        assert!(Solution::flip_equiv(root1, root2));
    }

    #[test]
    fn test_flip_equiv_3() {
        let root1 = TreeNode::from_string("");
        let root2 = TreeNode::from_string("1");
        assert!(!Solution::flip_equiv(root1, root2));
    }
}
