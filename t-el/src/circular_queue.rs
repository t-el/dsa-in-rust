// cercular queue implementation
// by (Github) @t-el  on jun 9, 2022
// Language: rust
// Path: src/circular_queue.rs

#[derive(Debug)]
#[derive(PartialEq)]
pub struct CircularQueue<T> {
    pub size : usize,
    rear : i32,
    front : i32,
    arr : Vec<T>
}

// impl CircularQueue
// impl<T :Copy> CircularQueue
impl<T :Copy> CircularQueue<T>{
    pub fn new(size : usize) -> Self {
        Self{
            size : size,
            rear : -1,
            front : -1,
            arr : vec![]
        }
    }  

    
    // enqueue() add  the element from the rear of the queue and returns it.
    // check if the queue is full
    // for the first element, set value of FRONT to 0
    // circularly increase the REAR index by 1 
    // if the rear is at the end, set the rear to 0

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
        if self.rear == self.size as i32 {
            self.rear = 0;
        }
        self.arr.push(element);
        Some(element) // return the element
    }

    // dequeue() removes the element from the front of the queue and returns it.
    // check if the queue is empty
    // for the first element, set value of REAR to 0
    // circularly decrease the FRONT index by 1
    // if the front is at the end, set the front to 0
    // for the last element, reset the values of FRONT and REAR to -1

    pub fn dequeue(&mut self) -> Option<T> {
        if self.arr.len() == 0 {
            return None;
        }
        if self.front == -1 {
            self.front = 0;
        }
        if self.rear == -1 {
            self.rear = 0;
        }
        let element = self.arr.remove(self.front as usize);
        self.front += 1;
        if self.front == self.size as i32 {
            self.front = 0;
        }
        if self.arr.len() == 0 {
            self.front = -1;
            self.rear = -1;
        }
        Some(element) // return the element
    }



 
    


  // check if the queue is full
    pub fn is_full(&self) -> bool {
        self.arr.len() == self.size
    }
   

   
      
} 

