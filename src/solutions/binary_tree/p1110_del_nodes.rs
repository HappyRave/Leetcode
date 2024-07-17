use std::cell::RefCell;
use std::rc::Rc;

use crate::solutions::Solution;

use super::TreeNode;
impl Solution {
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn del_nodes_helper(
            root: &Option<Rc<RefCell<TreeNode>>>,
            to_delete: &Vec<i32>,
            result: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            match root {
                None => None,
                Some(node) => {
                    let mut node = node.borrow_mut();
                    node.left = del_nodes_helper(&node.left, to_delete, result);
                    node.right = del_nodes_helper(&node.right, to_delete, result);
                    if to_delete.contains(&node.val) {
                        if let Some(left) = &node.left {
                            result.push(Some(Rc::clone(left)));
                        }
                        if let Some(right) = &node.right {
                            result.push(Some(Rc::clone(right)));
                        }
                        return None;
                    }
                    return Some(Rc::clone(root.as_ref().unwrap()));
                }
            }
        }

        let mut result = vec![];
        del_nodes_helper(&root, &to_delete, &mut result);
        if !to_delete.contains(&root.as_ref().unwrap().borrow().val) {
            result.push(root);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::binary_tree::BinaryTreeExt;

    use super::*;

    #[test]
    fn test_del_nodes() {
        let root = vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ]
        .into_tree();
        let to_delete = vec![3, 5];
        let mut result = Solution::del_nodes(root, to_delete);
        let expected_tree = vec![Some(1), Some(2), None, Some(4)].into_tree();
        let expected_tree_2 = vec![Some(6)].into_tree();
        let expected_tree_3 = vec![Some(7)].into_tree();
        let mut expected = vec![expected_tree, expected_tree_2, expected_tree_3];

        result.sort_by(|a, b| TreeNode::tree_size(a).cmp(&TreeNode::tree_size(b)));
        expected.sort_by(|a, b| TreeNode::tree_size(a).cmp(&TreeNode::tree_size(b)));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_del_nodes_2() {
        let root = vec![Some(1), Some(2), Some(4), None, Some(3)].into_tree();
        let to_delete = vec![3];
        let expected_tree = vec![Some(1), Some(2), Some(4)].into_tree();
        let expected = vec![expected_tree];
        let result = Solution::del_nodes(root, to_delete);
        assert_eq!(result, expected);
    }
}
