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
    use crate::solutions::binary_tree::OptionTreeNodeExt;

    use super::*;

    #[test]
    fn test_del_nodes() {
        let root = TreeNode::from_string("1,2,3,4,5,6,7");
        let to_delete = vec![3, 5];
        let mut result = Solution::del_nodes(root, to_delete);
        let expected_tree = TreeNode::from_string("1,2,null,4");
        let expected_tree_2 = TreeNode::from_string("6");
        let expected_tree_3 = TreeNode::from_string("7");
        let mut expected = vec![expected_tree, expected_tree_2, expected_tree_3];

        result.sort_by_key(|a| a.get_size());
        expected.sort_by_key(|a| a.get_size());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_del_nodes_2() {
        let root = TreeNode::from_string("1,2,4,null,3");
        let to_delete = vec![3];
        let expected_tree = TreeNode::from_string("1,2,4");
        let expected = vec![expected_tree];
        let result = Solution::del_nodes(root, to_delete);
        assert_eq!(result, expected);
    }
}
