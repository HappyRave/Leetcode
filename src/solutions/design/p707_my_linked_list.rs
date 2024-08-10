struct MyLinkedList {
    head: Option<Box<Node>>,
    size: i32,
}

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */
impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList {
            head: None,
            size: 0,
        }
    }

    fn get(&self, index: i32) -> i32 {
        if index < 0 || index >= self.size {
            return -1;
        }
        let mut current = &self.head;
        for _ in 0..index {
            current = &current.as_ref().unwrap().next;
        }
        current.as_ref().unwrap().val
    }

    fn add_at_head(&mut self, val: i32) {
        self.add_at_index(0, val)
    }

    fn add_at_tail(&mut self, val: i32) {
        self.add_at_index(self.size, val)
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index < 0 || index > self.size {
            return;
        }
        let mut new_node = Box::new(Node { val, next: None });
        if self.size == 0 {
            self.head = Some(new_node);
        } else if index == 0 {
            new_node.next = self.head.take();
            self.head = Some(new_node);
        } else {
            let mut current = &mut self.head;
            for _ in 0..index - 1 {
                current = &mut current.as_mut().unwrap().next;
            }
            new_node.next = current.as_mut().unwrap().next.take();
            current.as_mut().unwrap().next = Some(new_node);
        }
        self.size += 1;
    }

    fn delete_at_index(&mut self, index: i32) {
        if index < 0 || index >= self.size {
            return;
        }
        if self.size == 0 {
        } else if index == 0 {
            self.head = self.head.as_mut().unwrap().next.take();
        } else {
            let mut current = &mut self.head;
            for _ in 0..index - 1 {
                current = &mut current.as_mut().unwrap().next;
            }
            current.as_mut().unwrap().next =
                current.as_mut().unwrap().next.as_mut().unwrap().next.take();
        }
        self.size -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p707_my_linked_list_t1() {
        let mut obj = MyLinkedList::new();
        obj.add_at_head(1);
        obj.add_at_tail(3);
        obj.add_at_index(1, 2);
        assert_eq!(obj.get(1), 2);
        obj.delete_at_index(1);
        assert_eq!(obj.get(1), 3);
        obj.delete_at_index(0);
        assert_eq!(obj.get(0), 3);
        obj.delete_at_index(0);
        assert_eq!(obj.get(0), -1);
    }
}
