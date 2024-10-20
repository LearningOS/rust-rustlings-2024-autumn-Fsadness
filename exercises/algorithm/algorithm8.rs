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

pub struct myStack<T>
{
	//TODO
    t:bool, // t = true: value in q1; t = false: value in q2
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
            t: true,
			q1: Queue::<T>::new(),
			q2: Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        match self.t {
            true => { // value in q1, q2 empty
                self.q2.enqueue(elem);
                while let Ok(data) = self.q1.dequeue() {
                    self.q2.enqueue(data)
                }
                self.t = false
            },
            false => { // value in q2, q1 empty
                self.q1.enqueue(elem);
                while let Ok(data) = self.q2.dequeue() {
                    self.q1.enqueue(data)
                }
                self.t = true
            }
        }
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        match self.t {
            true => { // value in q1, q2 empty
                match self.q1.dequeue() {
                    Ok(data) => Ok(data),
                    Err(_) => Err("Stack is empty")
                }
            },
            false => { // value in q2, q1 empty
                match self.q2.dequeue() {
                    Ok(data) => Ok(data),
                    Err(_) => Err("Stack is empty")
                }
            }
        }
    }
    pub fn is_empty(&self) -> bool {
		//TODO
        self.q1.is_empty() && self.q2.is_empty()
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