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
    use crate::solutions::binary_tree::BinaryTreeExt;

    use super::*;

    #[test]
    fn test_balance_bst() {
        assert_eq!(Solution::balance_bst(None), None);
    }

    #[test]
    fn test_balance_bst_2() {
        let tree = vec![
            Some(1),
            None,
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            None,
        ]
        .into_tree();
        let result = Solution::balance_bst(tree);
        assert!(result.clone().unwrap().borrow().is_balanced());
        assert!(result.unwrap().borrow().is_search_tree());
    }

    #[test]
    fn test_balance_bst_3() {
        let tree = vec![Some(2), None, Some(1), None, Some(3)].into_tree();
        let result = Solution::balance_bst(tree.clone());
        assert!(result.clone().unwrap().borrow().is_balanced());
        assert!(result.clone().unwrap().borrow().is_search_tree());
    }

    #[test]
    fn test_balance_bst_4() {
        let tree = vec![Some(2), Some(1), Some(3)].into_tree();
        let result = Solution::balance_bst(tree.clone());
        assert!(result.clone().unwrap().borrow().is_balanced());
        assert!(result.clone().unwrap().borrow().is_search_tree());
        assert_eq!(tree, result);
    }
}
