// stack implementation in rust 
// by (Github) @t-el  on jun 9, 2022
// Language: rust
// Path: src/stack.rs



#[derive(Debug)]
#[derive(PartialEq)]
pub struct Stack<T> {
      pub size : usize,
      arr : Vec<T>
}


impl<T :Copy> Stack <T>{
   
  pub fn new (size:usize) -> Self  {
      Self{
        size : size,
        arr: vec![]
      }
  }

// push() add  the element from the top of the stack and returns it.
// push() returns None if the stack is full.

    pub fn push(&mut self,element : T) -> Option<T> {
        if self.arr.len() == self.size {
            return None;
        }
        self.arr.push(element);
        Some(element) // return the element

    }

    // push example
    // ---------------------------------
    //  0 1 2 3 4 5 6 7 8 9
    
    // push(1)
    // -----------------------------------
    //  0 1 2 3 4 5 6 7 8 9
    //  |1| | | | | | | | |
    
    // push(3)
    // ----------------------------------
    //  0 1 2 3 4 5 6 7 8 9
    //  |1|3| | | | | | | |

    // push(5)
    // ----------------------------------
    //  0 1 2 3 4 5 6 7 8 9
    //  |1|3|5| | | | | | | |


 // pop() removes the element from the top of the stack and returns it.
 // pop() returns None if the stack is empty.
 
    pub fn pop(&mut self) -> Option<T> {
        if self.arr.is_empty() {
            return None;
        }
        Some(self.arr.pop().unwrap())
    }
    
  // pop example
    // ---------------------------------
    //  0 1 2 3 4 5 6 7 8 9
    //  |1|3|5| | | | | | | |
    // pop()    
    // -----------------------------------
    //  0 1 2 3 4 5 6 7 8 9
    //  |1|3| | | | | | | | |
    // pop()
    // -----------------------------------
    //  0 1 2 3 4 5 6 7 8 9
    //  |1| | | | | | | | | |
 

    
  
// peek() returns the element from the top of the stack without removing it.
// peek() returns None if the stack is empty.

    pub fn peek(&self) -> Option<&T> {
        if self.arr.is_empty() {
            return None;
        }
        Some(&self.arr[self.arr.len()-1])
    }

    // peek example
    // ---------------------------------
    //  0 1 2 3 4 5 6 7 8 9
    //  |1|3|5| | | | | | | |
    // peek()
    // returns Some(&5)



// peek_at returns the element with the given index from the top of the stack without removing it.
// peek_at() returns None if the stack is empty or if the index is out of bounds.


    pub fn peek_at(&self,index:usize) -> Option<&T> {
        if self.arr.is_empty() {
            return None;
        }
        if index >= self.arr.len() {
            return None;
        }
        Some(&self.arr[index])
    }
    
    // peek_at example
    // ---------------------------------
    //  0 1 2 3 4 5 6 7 8 9
    //  |1|3|5| | | | | | | |

    // peek_at(0)
    // returns 1 
    // peek_at(1)
    // returns 3
    // peek_at(2)
    // returns 5
    

 // clear() removes all elements from the stack.
 // clear() returns None if the stack is empty.
 // clear() returns Some(()) if the stack is not empty.
 
    pub fn clear(&mut self) -> Option<()> {
        if self.arr.is_empty() {
            return None;
        }
        self.arr.clear();
        Some(())
    }
    

// len() returns the number of elements in the stack.
// len() returns 0 if the stack is empty.
// len() returns Some(usize) if the stack is not empty.

    pub fn len(&self) -> usize {
        self.arr.len()
    }
   


}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_new() {
    let mut list:Stack<u32> =  Stack::new(3);
    assert_eq!(list.len() ,0);
    assert_eq!(list.size ,3);

    }

    #[test]
    fn test_push() {
    let mut list =  Stack::new(3);

    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(list.len(),3);
    
    }

    #[test]
    fn test_pop() {
    let mut list :Stack<i8> = Stack::new(3);
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(list.pop().unwrap(),3); 
    assert_eq!(list.pop().unwrap(),2);
    assert_eq!(list.pop().unwrap(),1);
    assert_eq!(list.pop(),None);

   
    
    }

    #[test]
    fn test_peek() {
    let mut list :Stack<i8> = Stack::new(3);
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(list.peek().unwrap(),&3);
    let mut emptylist :Stack<i8> = Stack::new(3);
    assert_eq!(emptylist.peek(),None);

    }

    #[test]
    fn test_peek_at() {
    let mut list :Stack<i8> = Stack::new(3);
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(list.peek_at(2).unwrap(),&3);
    assert_eq!(list.peek_at(1).unwrap(),&2);
    assert_eq!(list.peek_at(0).unwrap(),&1);
    assert_eq!(list.peek_at(3),None);
    
    }
    
    

    #[test]
    fn test_clear() {
    let mut list :Stack<i8> = Stack::new(3);
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(list.clear(),Some(()));
    assert_eq!(list.len(),0);
    let mut emptylist :Stack<i8> = Stack::new(3);
    assert_eq!(emptylist.clear(),None);

    }

    
}