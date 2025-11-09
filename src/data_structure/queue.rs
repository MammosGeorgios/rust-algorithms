
#[allow(dead_code)]
#[derive(Clone)]
pub struct QNode {
    pub value: i32,
    pub next: Option<Box<QNode>>,
}

impl QNode {
    pub fn get_value(self) -> i32 {
        self.value
    }
}

pub struct MyQueue {
    length: usize,
    head: Option<Box<QNode>>,
    tail: Option<Box<QNode>>,
}

impl MyQueue {
    pub(crate) fn new() -> MyQueue {
        MyQueue {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub fn get_length(&self) -> usize {
        self.length
    }

    pub fn enqueue(&mut self, item: i32) {}

    pub fn deque(mut self) -> Option<i32> {
        if self.head.is_none() {
            return None;
        }
        
        let head_before_deque =  self.head.unwrap();
        let head_node_before_deque = *head_before_deque;

        let new_head = head_node_before_deque.next;
        self.head = new_head;
        
        self.length = self.length - 1;

        Some(head_node_before_deque.value)
    }

    pub fn peek(&mut self) -> Option<i32> {
        if self.head.is_none() {
            return None;
        }

        let current_head = self.head.unwrap();
        let data = *current_head;
        
        Some(data.get_value())
    }
}

#[cfg(test)]
mod test {
    use crate::data_structure::queue::MyQueue;

    #[test]
    fn init_empty_queue() {
        let _my_queue = MyQueue::new();
    }
    #[test]
    fn empty_queue_length_is_zero() {
        let my_queue = MyQueue::new();
        assert_eq!(
            my_queue.get_length(),
            0usize,
            "We expect length to be zero for empty queue"
        )
    }

    #[test]
    fn add_first_element() {
        let mut my_queue = MyQueue::new();
        my_queue.enqueue(1);

        assert_eq!(my_queue.get_length(), 1, "Expected queue length = 1")
    }
    
    #[test]
    fn fem_test(){
        
        let mut queue = MyQueue::new();
        queue.enqueue(5);
        queue.enqueue(7);
        queue.enqueue(9);
        
        assert_eq!(queue.peek(), Some(5));
        
        assert_eq!(queue.deque(), Some(5));
        assert_eq!(queue.get_length(), 2);
        assert_eq!(queue.deque(), Some(5))
        
    }
}
