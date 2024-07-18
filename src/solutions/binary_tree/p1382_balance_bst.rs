use std::cell::RefCell;
use std::rc::Rc;

use crate::solutions::Solution;

use super::TreeNode;
impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
            if let Some(node) = root {
                let node_ref = node.borrow();
                dfs(node_ref.left.clone(), values);
                values.push(node_ref.val);
                dfs(node_ref.right.clone(), values);
            }
        }

        fn build_tree(values: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if values.is_empty() {
                return None;
            }
            let mid = values.len() / 2;
            let mut node = TreeNode::new(values[mid]);
            node.left = build_tree(&values[..mid]);
            node.right = build_tree(&values[mid + 1..]);
            Some(Rc::new(RefCell::new(node)))
        }

        let mut values = vec![];
        dfs(root, &mut values);
        values.sort_unstable();
        build_tree(&values)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::binary_tree::OptionTreeNodeExt;

    use super::*;

    #[test]
    fn test_balance_bst() {
        assert_eq!(Solution::balance_bst(None), None);
    }

    #[test]
    fn test_balance_bst_2() {
        let tree = TreeNode::from_string("1,null,2,null,3,null,4,null,null");
        let result = Solution::balance_bst(tree);
        assert!(result.is_balanced());
        assert!(result.is_search_tree());
    }

    #[test]
    fn test_balance_bst_3() {
        let tree = TreeNode::from_string("2,null,1,null,3");
        let result = Solution::balance_bst(tree);
        assert!(result.is_balanced());
        assert!(result.is_search_tree());
    }

    #[test]
    fn test_balance_bst_4() {
        let tree = TreeNode::from_string("2,1,3");
        let result = Solution::balance_bst(tree.clone());
        assert!(result.is_balanced());
        assert!(result.is_search_tree());
        assert_eq!(tree, result);
    }
}
