struct MinStack {
    valueStack: Vec<i32>,
    minStack: Vec<i32>,
    minValue: i32
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

use std::i32::MAX;

impl MinStack {
    fn new() -> Self {
        MinStack { 
            minValue: MAX,
            valueStack: Vec::new(),
            minStack: Vec::new(),
        }
    }
    
    fn push(&mut self, val: i32) {
        self.valueStack.push(val);
        if self.minStack.is_empty() || val <= self.get_min(){
            self.minStack.push(val);
            self.minValue = val;
        }
    }
    
    fn pop(&mut self) {
        if self.valueStack.is_empty(){
            return 
        }
        if self.valueStack.pop().unwrap() == self.get_min(){
            self.minStack.pop();
        }
    }
    
    fn top(&self) -> i32 {
        return *self.valueStack.last().unwrap();
    }
    
    fn get_min(&self) -> i32 {
        return *self.minStack.last().unwrap();
    }
}

