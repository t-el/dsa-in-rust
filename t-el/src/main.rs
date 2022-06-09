mod stack;
use std::{time::Duration, thread::sleep};
use stack::Stack;

fn main() {
    let mut stack = Stack::new(10);
   
    for i in 0..10 {
        stack.push(i);
        display_stack(&stack, i+1);
        // sleep for 1 second
        sleep(Duration::from_secs(1));
    }
    println!("");

  
 
    fn display_stack(stack:&Stack<i32>,size : i32) {
        // clear stdout before printing
        print!("\x1b[2J\x1b[1;1H");
        for i in 0..size {
            println!("|{:?}| length : {}", stack.peek_at(i as usize).unwrap(),stack.len());
        }
    }
  

    
    }
  
