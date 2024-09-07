pub mod p1367_is_sub_path;
pub mod p2058_nodes_between_critical_points;
pub mod p2181_merge_nodes;
pub mod p234_is_palindrome_list;
pub mod p2487_remove_nodes;
pub mod p2816_double_it;
pub mod p2_add_two_numbers_linked;
pub mod p3217_modified_list;
pub mod p61_rotate_right;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

trait ListExt {
    fn into_list(self) -> Option<Box<ListNode>>;
}

impl ListExt for Vec<i32> {
    fn into_list(self) -> Option<Box<ListNode>> {
        let mut head = None;
        for val in self.into_iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }
        head
    }
}
