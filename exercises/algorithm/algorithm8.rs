/*
	queue
	This question requires you to use queues to implement the functionality of the stack
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

pub struct myStack<T>
{
	//TODO
    last_push: usize,
	q: [Queue<T>; 2]
}

impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			last_push: 0,
            q: [Queue::<T>::new(), Queue::<T>::new()],
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        self.q[self.last_push].enqueue(elem);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        let mut other = (self.last_push + 1) % 2;

        if self.q[self.last_push].is_empty() && !self.q[other].is_empty() {
            std::mem::swap(&mut self.last_push, &mut other);
        }

        while self.q[self.last_push].size() > 1 {
            let elem = self.q[self.last_push].dequeue().unwrap();
            self.q[other].enqueue(elem);
        }

        self.q[self.last_push].dequeue().map_err(|e| {
            "Stack is empty"
        })
    }
    pub fn is_empty(&self) -> bool {
		//TODO
        self.q[0].is_empty() && self.q[1].is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
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