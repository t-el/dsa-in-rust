//  Queue implimetation in rust
// by (Github) @t-el  on jun 9, 2022
// Language: rust
// Path: src/queue.rs

// twitter api implimentation in rust
// by (Github) @t-el  on jun 9, 2022
// Language: rust
// Path: src/twitter.rs



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
    // enqueue() add  the element from the rear of the queue and returns it.
    // enqueue() returns None if the queue is full.
    // enqueue() set the front to 0 if the queue is empty.
    // enqueue() set the rear to 0 if the queue is empty.
    // enqueue() increases the rear by 1.

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


  
   
    // enqueue example 
    // front =-1
    // rear =-1
    // ---------------------------------
    //  0 1 2 3 4 5 6 7 8 9

    // enqueue(1)
    // front =0
    // rear =1
    // -----------------------------------
    //  0 1 2 3 4 5 6 7 8 9
    //  |1| | | | | | | | |

    // enqueue(3)
    // front =0
    // rear =2
    // ----------------------------------
    //  0 1 2 3 4 5 6 7 8 9
    //  |1|3| | | | | | | |

    // enqueue(5)
    // front =0
    // rear =3
    // ----------------------------------
    //  0 1 2 3 4 5 6 7 8 9
    //  |1|3|5| | | | | | | |
    

    


    // dequeue() removes the element from the front of the queue and returns it. 
    // If the queue is empty, it returns None.
    // dequeue() set the front to -1 if the queue is empty.
    // dequeue() set the rear to -1 if the queue is empty.
    // dequeue() increases the front by 1 if the queue is not empty.

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

    // isFull() returns true if queue is full else false
    // queue is full when the rear is equal to the size of the queue.
    // isFull() returns false if the queue is empty.

    pub fn isfull(&self) -> bool {
        if self.arr.len() == self.size {
            return true;
        }
        false
    }
   
    
    

    // queue isEmpty() returns true if queue is empty else false
    // queue is empty when the front is equal to the rear. 
    // queue is empty when the front is equal to -1.
    // queue is empty when the rear is equal to -1.

    pub fn isempty(&self) -> bool {
        self.front == self.rear ||  self.front == -1
    }
    
    

 // clear() removes all elements from the stack.
 // clear() returns None if the stack is empty.
 // clear() returns Some(()) if the stack is not empty.
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
    
   // return the length of the queue
   // if the queue is empty, return 0 
   // length of array from front to rear

    pub fn len(&self) -> usize {
        if self.isempty() {
            0
        }else{
            (self.rear - self.front) as usize
        }

    }
}

// implement the iterator trait for the queue struct to iterate over the queue elements in FIFO order. 
// The iterator should return a reference to the element.
// The iterator should return None if the queue is empty.
// make an index variable to keep track of the current element.
// make a counter variable to keep track of the number of elements in the queue.
// make a loop to iterate over the queue


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
    fn test_clear() {
    let mut list :Queue<i8> = Queue::new(3);
    list.enqueue(1); // returns Some(1) if the queue is not full
    list.enqueue(2); // returns Some(2) if the queue is not full
    list.enqueue(3); // returns Some(3) if the queue is not full
    assert_eq!(list.clear().unwrap(),()); // returns Some(()) if the queue is not empty
    assert_eq!(list.clear().is_none(),true); // returns None if the queue is empty
    }
    

}
