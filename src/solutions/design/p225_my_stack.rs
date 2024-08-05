struct MyStack {
    queue: Vec<i32>,
    last: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */
impl MyStack {
    fn new() -> Self {
        MyStack {
            queue: Vec::new(),
            last: 0,
        }
    }

    fn push(&mut self, x: i32) {
        self.queue.push(x);
        self.last += 1;
    }

    fn pop(&mut self) -> i32 {
        self.last -= 1;
        self.queue.remove(self.last)
    }

    fn top(&self) -> i32 {
        self.queue[self.last - 1]
    }

    fn empty(&self) -> bool {
        self.last == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_stack() {
        let mut obj = MyStack::new();
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.top(), 2);
        assert_eq!(obj.pop(), 2);
        assert!(!obj.empty());
        assert_eq!(obj.pop(), 1);
        assert!(obj.empty());
    }
}
