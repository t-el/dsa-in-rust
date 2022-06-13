// Queue implimetation in rust
// by (Github) @t-el  on jun 9, 2022
//Language: rust
// Path: src/queue.rs




#[derive(Debug)]
#[derive(PartialEq)]
pub struct Queue<T> {
       pub size : usize,
       rear : i32,
       front : i32,
       arr : Vec<T>
}


impl<T :Copy> Queue <T>{
   
    pub fn new(size : usize) -> Self {
        Self{
            size : size,
            rear : -1,
            front : -1,
            arr : vec![]
        }
    }
    /// enqueue() add  the element from the rear of the queue and returns it.
    /// enqueue() returns None if the queue is full.
    /// # example
    /// ```
    /// let mut q = Queue::new(5);
    /// assert_eq!(q.enqueue(1), Some(1));
    /// assert_eq!(q.enqueue(2), Some(2));
    /// assert_eq!(q.enqueue(3), Some(3));
    /// ```
    /// # Panics
    /// Panics if the queue is full.
    /// # Example
    /// ```
    /// let mut q = Queue::new(5);
    /// assert_eq!(q.enqueue(1), Some(1));
    /// assert_eq!(q.enqueue(2), Some(2));
    /// assert_eq!(q.enqueue(3), Some(3));
    /// assert_eq!(q.enqueue(4), Some(4));
    /// assert_eq!(q.enqueue(5), None);
    /// ```
    pub fn enqueue(&mut self,element : T) -> Option<T> {
        if self.arr.len() == self.size {
            return None;
        }
        if self.front == -1 {
            self.front = 0;
        }
        if self.rear == -1 {
            self.rear = 0;
        }
        self.rear += 1;
        self.arr.push(element);
        Some(element) // return the element
    }



    /// dequeue() removes the element from the front of the queue and returns it.
    /// dequeue() returns None if the queue is empty.
    /// # example
    /// ```
    /// let mut q = Queue::new(5);
    /// assert_eq!(q.enqueue(1), Some(1));
    /// assert_eq!(q.enqueue(2), Some(2));
    /// assert_eq!(q.enqueue(3), Some(3));
    /// assert_eq!(q.dequeue(), Some(1));
    /// assert_eq!(q.dequeue(), Some(2));
    /// assert_eq!(q.dequeue(), Some(3));
    /// ```
    /// # Panics
    /// Panics if the queue is empty.
    /// # Example
    /// ```
    /// let mut q = Queue::new(5);
    /// assert_eq!(q.enqueue(1), Some(1));
    /// assert_eq!(q.enqueue(2), Some(2));
    /// assert_eq!(q.enqueue(3), Some(3));
    /// assert_eq!(q.dequeue(), Some(1));
    /// assert_eq!(q.dequeue(), Some(2));
    /// assert_eq!(q.dequeue(), Some(3));
    /// assert_eq!(q.dequeue(), None);
    /// ```

    pub fn dequeue(&mut self) -> Option<T> {
       
        if self.arr.is_empty() {
            return None;
        }
        if self.front == self.rear {
            let temp = self.arr[self.front as usize];
            self.front = -1;
            self.rear = -1;
            Some(temp)
        }
        else {
            let temp = self.arr[self.front as usize];
            self.front += 1;
            Some(temp)
        }
    }

    /// isfull() returns true if the queue is full.
    /// isfull() returns false if the queue is not full.
    /// # example
    /// ```
    /// let mut q = Queue::new(5);
    /// assert_eq!(q.isfull(), false);
    /// assert_eq!(q.enqueue(1), Some(1));
    /// assert_eq!(q.enqueue(2), Some(2));
    /// assert_eq!(q.enqueue(3), Some(3));
    /// assert_eq!(q.enqueue(4), Some(4));
    /// assert_eq!(q.enqueue(5), None);
    /// assert_eq!(q.isfull(), true);
    /// ```

    pub fn isfull(&self) -> bool {
        if self.arr.len() == self.size {
            return true;
        }
        false
    }
   

  /// is_empty() returns true if the queue is empty.
    /// is_empty() returns false if the queue is not empty.
    /// # example
    /// ```
    /// let mut q = Queue::new(5);
    /// assert_eq!(q.is_empty(), true);
    /// assert_eq!(q.enqueue(1), Some(1));
    /// assert_eq!(q.enqueue(2), Some(2));
    /// assert_eq!(q.enqueue(3), Some(3));
    /// assert_eq!(q.is_empty(), false);
    /// ```

    pub fn isempty(&self) -> bool {
        self.front == self.rear ||  self.front == -1
    }
    
  
    /// peek() returns the element at the front of the queue without removing it.
    /// peek() returns None if the queue is empty.
    /// # example
    /// ```
    /// let mut q = Queue::new(5);
    /// assert_eq!(q.peek(), None);
    /// assert_eq!(q.enqueue(1), Some(1));
    /// assert_eq!(q.enqueue(2), Some(2));
    /// assert_eq!(q.enqueue(3), Some(3));
    /// assert_eq!(q.peek(), Some(1));
    /// ```
    /// # Panics
    /// Panics if the queue is empty.
    /// # Example
    /// ```
    /// let mut q = Queue::new(5);
    /// assert_eq!(q.peek(), None);
    /// assert_eq!(q.enqueue(1), Some(1));
    /// assert_eq!(q.enqueue(2), Some(2));
    /// assert_eq!(q.enqueue(3), Some(3));
    /// assert_eq!(q.peek(), Some(1));
    /// ```
    /// 
    pub fn peek(&self) -> Option<T> {
        if self.arr.is_empty() {
            return None;
        }
        Some(self.arr[self.front as usize])
    }

    /// capacity() returns the capacity of the queue.
    /// # example
    /// ```
    /// let mut q = Queue::new(5);
    /// assert_eq!(q.capacity(), 5);
    /// ```
    
    pub fn capacity(&self) -> usize {
        self.size
    }

    /// clear() removes all the elements from the queue.
    /// # example
    /// ```
    /// let mut q = Queue::new(5);
    /// assert_eq!(q.enqueue(1), Some(1));
    /// assert_eq!(q.enqueue(2), Some(2));
    /// assert_eq!(q.enqueue(3), Some(3));
    /// assert_eq!(q.clear(), None);
    /// ```
    
    
    pub fn clear(&mut self) -> Option<()> {
        if self.isempty(){
            None
        }else{
            self.front = -1;
            self.rear = -1;
            self.arr.clear();
            Some(())
        }
    }
    
   /// len() return the length of the queue
   /// # example
   /// ```
   /// let mut q = Queue::new(5);
   /// assert_eq!(q.len(), 0);
   /// assert_eq!(q.enqueue(1), Some(1));
   /// assert_eq!(q.enqueue(2), Some(2));
   /// assert_eq!(q.enqueue(3), Some(3));
   /// assert_eq!(q.len(), 3);
   /// ```

    pub fn len(&self) -> usize {
        if self.isempty() {
            0
        }else{
            (self.rear - self.front) as usize
        }

    }
}



impl<T : Copy> Iterator for Queue<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.isempty() {
            None
        }else{
            let temp = self.arr[self.front as usize];
            self.front += 1;
            Some(temp)
        }

    }

}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_new() {
    let mut list:Queue<u32> =  Queue::new(3);
    assert_eq!(list.len() ,0);
    assert_eq!(list.size ,3);
    assert_eq!(list.front ,-1);
    assert_eq!(list.rear ,-1);

    }

    #[test]
    fn test_enqueue() {
    let mut list:Queue<u32> =  Queue::new(3);
    assert_eq!(list.enqueue(2).unwrap(),2); // return the element 2 
    assert_eq!(list.enqueue(2).unwrap(),2); // return the element 2
    assert_eq!(list.enqueue(2).unwrap(),2); // return the element 2
    assert_eq!(list.enqueue(2).is_none(),true); // return None if the queue is full
    assert_eq!(list.len() ,3);
    assert_eq!(list.isfull(),true);
   
    }

    #[test]
    fn test_dequeue() {
    let mut list:Queue<u32> =  Queue::new(3);
    assert_eq!(list.enqueue(2).unwrap(),2); // return the element 2
    assert_eq!(list.enqueue(2).unwrap(),2); // return the element 2
    assert_eq!(list.enqueue(2).unwrap(),2); // return the element 2
    assert_eq!(list.dequeue().unwrap(),2); // return the element 2
    assert_eq!(list.dequeue().unwrap(),2); // return the element 2
    assert_eq!(list.dequeue().unwrap(),2); // return the element 2
    //assert_eq!(list.dequeue().is_none(),true); // return None if the queue is empty
    assert_eq!(list.len() ,0);
    assert_eq!(list.isempty(),true);
   
    }
   
  
    #[test]
    fn test_isempty() {
        let mut list:Queue<u32> =  Queue::new(3);
        assert_eq!(list.isempty(),true);
        list.enqueue(2); // return the element 2
        assert_eq!(list.isempty(),false);
        list.dequeue(); // return the element 2
        assert_eq!(list.isempty(),true);
       
    }

    #[test]
    fn test_isfull() {
        let mut list:Queue<u32> =  Queue::new(3);
        assert_eq!(list.isfull(),false); // queue is not full
        list.enqueue(2); // returns Some(2) if the queue is not full
        assert_eq!(list.isfull(),false);    
    }
        
     
    #[test]
    fn test_capacity() {
    let mut list :Queue<i8> = Queue::new(3);
    assert_eq!(list.capacity(),3);
    }

    #[test]
    fn test_clear() {
    let mut list :Queue<i8> = Queue::new(3);
    list.enqueue(1); // returns Some(1) if the queue is not full
    list.enqueue(2); // returns Some(2) if the queue is not full
    list.enqueue(3); // returns Some(3) if the queue is not full
    assert_eq!(list.clear().unwrap(),()); // returns Some(()) if the queue is not empty
    assert_eq!(list.clear().is_none(),true); // returns None if the queue is empty
    }
    

}
