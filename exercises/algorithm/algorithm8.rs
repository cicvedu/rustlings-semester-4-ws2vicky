/*
    queue
    This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct MyStack<T> {
    //TODO
    // q1: Queue<T>,
    // q2: Queue<T>,
    vec: Vec<T>,
}
impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            //TODO
            // q1: Queue::<T>::new(),
            // q2: Queue::<T>::new(),
            vec: Vec::new(),
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        // self.q1.enqueue(elem);
        self.vec.push(elem);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        // Err("Stack is empty")
        // while self.q1.size() > 1 {
        //     if let Ok(value) = self.q1.dequeue() {
        //         self.q2.enqueue(value);
        //     }
        // }
        // let result = self.q1.dequeue();
        // std::mem::swap(&mut self.q1, &mut self.q2);
        // result

        // while self.q1.size() > 1 {
        //     if let Ok(value) = self.q1.dequeue() {
        //         self.q2.enqueue(value);
        //     }
        // }
        // let result;
        // {
        //     let temp_q1 = &mut self.q1;
        //     result = temp_q1.dequeue();
        // }
        // std::mem::swap(&mut self.q1, &mut self.q2);
        // result

        // while self.q1.size() > 1 {
        //     if let Ok(value) = self.q1.dequeue() {
        //         self.q2.enqueue(value);
        //     }
        // }
        // std::mem::swap(&mut self.q1, &mut self.q2);
        // self.q1.dequeue()

        // while self.q1.size() > 1 {
        //     if let Ok(value) = self.q1.dequeue() {
        //         self.q2.enqueue(value);
        //     }
        // }
        // std::mem::swap(&mut self.q1, &mut self.q2);
        // if self.q1.is_empty() {
        //     Err("Stack is empty")
        // } else {
        //     self.q1.dequeue()
        // }
        // while self.q1.size() > 1 {
        //     if let Ok(value) = self.q1.dequeue() {
        //         self.q2.enqueue(value);
        //     }
        // }
        // let result = self.q1.dequeue();
        // while let Ok(value) = self.q2.dequeue() {
        //     self.q1.enqueue(value);
        // }
        // result

        // while self.q1.size() > 1 {
        //     if let Ok(value) = self.q1.dequeue() {
        //         self.q2.enqueue(value);
        //     }
        // }
        // let result = self.q1.dequeue();
        // let mut temp_q2 = Queue::<T>::new();
        // std::mem::swap(&mut self.q2, &mut temp_q2);
        // while let Ok(value) = temp_q2.dequeue() {
        //     self.q1.enqueue(value);
        // }
        // result

        // while self.q1.size() > 1 {
        //     if let Ok(value) = self.q1.dequeue() {
        //         self.q2.enqueue(value);
        //     }
        // }
        // let result = self.q1.dequeue();
        // let mut temp_q2 = Queue::<T>::new();
        // std::mem::swap(&mut self.q2, &mut temp_q2);
        // while let Ok(value) = temp_q2.dequeue() {
        //     self.q1.enqueue(value);
        // }
        // result

        match self.vec.pop() {
            Some(t) => Ok(t),
            None => Err("Stack is empty"),
        }
    }
    pub fn is_empty(&self) -> bool {
        //TODO
        // true
        // self.q1.is_empty()
        self.vec.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = MyStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}
