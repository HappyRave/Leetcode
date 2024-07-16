use std::cell::RefCell;
use std::rc::Rc;

use crate::solutions::Solution;

use super::TreeNode;

impl Solution {
    pub fn get_directions(
        root: Option<Rc<RefCell<TreeNode>>>,
        start_value: i32,
        dest_value: i32,
    ) -> String {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, target: i32, path: &mut Vec<char>) -> bool {
            if let Some(node) = node {
                let n = node.borrow();
                if n.val == target {
                    return true;
                }
                path.push('L');
                if dfs(n.left.clone(), target, path) {
                    return true;
                }
                path.pop();
                path.push('R');
                if dfs(n.right.clone(), target, path) {
                    return true;
                }
                path.pop();
            }
            false
        }

        let mut path_to_start = Vec::new();
        dfs(root.clone(), start_value, &mut path_to_start);
        let mut path_to_dest = Vec::new();
        dfs(root, dest_value, &mut path_to_dest);

        while !path_to_start.is_empty()
            && !path_to_dest.is_empty()
            && path_to_start[0] == path_to_dest[0]
        {
            path_to_start.remove(0);
            path_to_dest.remove(0);
        }

        path_to_start
            .iter()
            .map(|_| 'U')
            .chain(path_to_dest)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::binary_tree::BinaryTreeExt;

    #[test]
    fn test_get_directions_basic() {
        let tree = vec![Some(5), Some(1), Some(2), Some(3), None, Some(6), Some(4)].into_tree();
        assert_eq!(Solution::get_directions(tree, 3, 6), "UURL".to_string()); // cspell: disable-line
    }

    #[test]
    fn test_get_directions_basic_2() {
        let tree = vec![Some(2), Some(1)].into_tree();
        assert_eq!(Solution::get_directions(tree, 2, 1), "L".to_string());
    }
}
