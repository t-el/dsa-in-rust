// stack implementation in rust 
// by (Github) @t-el  on jun 9, 2022
// Language: rust
// Path: src/stack.rs

#![allow(dead_code, unused_variables)]

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

/// push() adds the element to the top of the stack and returns it.
/// push() returns None if the stack is full.
/// # example
/// ```
/// let mut s = Stack::new(5);
/// assert_eq!(s.push(1), Some(1));
/// assert_eq!(s.push(2), Some(2));
/// assert_eq!(s.push(3), Some(3));
/// ```
/// # Panics
/// Panics if the stack is full.
/// # Example
/// ```
/// let mut s = Stack::new(5);
/// assert_eq!(s.push(1), Some(1));
/// assert_eq!(s.push(2), Some(2));
/// assert_eq!(s.push(3), Some(3));
/// assert_eq!(s.push(4), Some(4));
/// assert_eq!(s.push(5), None);
/// ```

    pub fn push(&mut self,element : T) -> Option<T> {
        if self.arr.len() == self.size {
            return None;
        }
        self.arr.push(element);
        Some(element) // return the element

    }



 ///  pop() removes the element from the top of the stack and returns it.
 /// pop() returns None if the stack is empty.
 /// # example
 /// ```
 /// let mut s = Stack::new(5);
 /// assert_eq!(s.push(1), Some(1));
 /// assert_eq!(s.push(2), Some(2));
 /// assert_eq!(s.push(3), Some(3));
 /// assert_eq!(s.pop(), Some(3));
 /// assert_eq!(s.pop(), Some(2));
 /// assert_eq!(s.pop(), Some(1));
 /// assert_eq!(s.pop(), None);
 /// ```
 /// # Panics
 /// Panics if the stack is empty.
 /// # Example
 /// ```
 /// let mut s = Stack::new(5);
 /// assert_eq!(s.push(1), Some(1));
 /// assert_eq!(s.push(2), Some(2));
 /// assert_eq!(s.push(3), Some(3));
 /// assert_eq!(s.push(4), Some(4));
 /// assert_eq!(s.push(5), None);
 /// ```
    pub fn pop(&mut self) -> Option<T> {
        if self.arr.len() == 0 {
            return None;
        }
        let element = self.arr.pop();
        Some(element.unwrap())

    }
 
    /// peek() returns the element at the top of the stack without removing it.
    /// peek() returns None if the stack is empty.
    /// # example
    /// ```
    /// let mut s = Stack::new(5);
    /// assert_eq!(s.push(1), Some(1));
    /// assert_eq!(s.push(2), Some(2));
    /// assert_eq!(s.push(3), Some(3));
    /// assert_eq!(s.peek(), Some(3));
    /// ```
    /// # Panics
    /// Panics if the stack is empty.
    /// # Example
    /// ```
    /// let mut s = Stack::new(5);
    /// assert_eq!(s.push(1), Some(1));
    /// assert_eq!(s.push(2), Some(2));
    /// assert_eq!(s.push(3), Some(3));
    /// assert_eq!(s.peek(), Some(3));
    /// ```

    pub fn peek(&self) -> Option<&T> {
        if self.arr.is_empty() {
            return None;
        }
        Some(&self.arr[self.arr.len()-1])
    }


/// peek_at() returns the element with the given index from the top of the stack without removing it.
/// peek_at() returns None if the stack is empty or if the index is out of bounds.
/// # example
/// ```
/// let mut s = Stack::new(5);
/// assert_eq!(s.push(1), Some(1));
/// assert_eq!(s.push(2), Some(2));
/// assert_eq!(s.push(3), Some(3));
/// assert_eq!(s.peek_at(0), Some(&3));
/// assert_eq!(s.peek_at(1), Some(&2));
/// assert_eq!(s.peek_at(2), Some(&1));
/// assert_eq!(s.peek_at(3), None);
/// ```
/// # Panics
/// Panics if the stack is empty or if the index is out of bounds.
/// # Example
/// ```
/// let mut s = Stack::new(5);
/// assert_eq!(s.push(1), Some(1));
/// assert_eq!(s.push(2), Some(2));
/// assert_eq!(s.push(3), Some(3));
/// assert_eq!(s.peek_at(0), Some(&3));
/// assert_eq!(s.peek_at(1), Some(&2));
/// assert_eq!(s.peek_at(2), Some(&1));
/// assert_eq!(s.peek_at(3), None);
/// ```



    pub fn peek_at(&self,index:usize) -> Option<&T> {
        if self.arr.is_empty() {
            return None;
        }
        if index >= self.arr.len() {
            return None;
        }
        Some(&self.arr[index])
    }
    

    

 /// clear() removes all elements from the stack.
 /// clear() returns None if the stack is empty.
 /// clear() returns Some(()) if the stack is not empty.
 /// # example
 /// ```
 /// let mut s = Stack::new(5);
 /// assert_eq!(s.push(1), Some(1));
 /// assert_eq!(s.push(2), Some(2));
 /// assert_eq!(s.push(3), Some(3));
 /// assert_eq!(s.clear(), Some(()));
 /// ```
 /// # Panics
 /// Panics if the stack is empty.
 /// # Example
 /// ```
 /// let mut s = Stack::new(5);
 /// assert_eq!(s.push(1), Some(1));
 /// assert_eq!(s.push(2), Some(2));
 /// assert_eq!(s.push(3), Some(3));
 /// assert_eq!(s.clear(), Some(()));
 /// ```
 
    pub fn clear(&mut self) -> Option<()> {
        if self.arr.is_empty() {
            return None;
        }
        self.arr.clear();
        Some(())
    }
    

/// len() returns the number of elements in the stack.
/// len() returns 0 if the stack is empty.
/// len() returns Some(usize) if the stack is not empty.
/// # example
/// ```
/// let mut s = Stack::new(5);
/// assert_eq!(s.push(1), Some(1));
/// assert_eq!(s.push(2), Some(2));
/// assert_eq!(s.push(3), Some(3));
/// assert_eq!(s.len(), Some(3));
/// ```
/// # Panics
/// Panics if the stack is empty.
/// # Example
/// ```
/// let mut s = Stack::new(5);
/// assert_eq!(s.push(1), Some(1));
/// assert_eq!(s.push(2), Some(2));
/// assert_eq!(s.push(3), Some(3));
/// assert_eq!(s.len(), Some(3));
/// ```

    pub fn len(&self) -> usize {
        self.arr.len()
    }
   

    /// is_empty() returns true if the stack is empty.
    /// is_empty() returns false if the stack is not empty.
    /// # example
    /// ```
    /// let mut s = Stack::new(5);
    /// assert_eq!(s.push(1), Some(1));
    /// assert_eq!(s.push(2), Some(2));
    /// assert_eq!(s.push(3), Some(3));
    /// assert_eq!(s.is_empty(), false);
    /// ```
    
    pub fn is_empty(&self) -> bool {
        self.arr.is_empty()
    }

    /// is_full() returns true if the stack is full.
    /// is_full() returns false if the stack is not full.
    /// # example
    /// ```
    /// let mut s = Stack::new(5);
    /// assert_eq!(s.push(1), Some(1));
    /// assert_eq!(s.push(2), Some(2));
    /// assert_eq!(s.push(3), Some(3));
    /// assert_eq!(s.is_full(), false);
    /// ```
    /// # Panics
    /// Panics if the stack is empty.
    /// # Example
    /// ```
    /// let mut s = Stack::new(5);
    /// assert_eq!(s.push(1), Some(1));
    /// assert_eq!(s.push(2), Some(2));
    /// assert_eq!(s.push(3), Some(3));
    /// assert_eq!(s.is_full(), false);
    /// ```
    
    pub fn is_full(&self) -> bool {
        self.arr.len() == self.size
    }

}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_new() {
    let  list:Stack<u32> =  Stack::new(3);
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
    let emptylist :Stack<i8> = Stack::new(3);
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