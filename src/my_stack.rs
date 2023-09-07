use std::cell::RefCell;

#[derive(Debug)]
pub struct MyStack<T> {
    pub stack: RefCell<Vec<T>>,
}
impl<T> MyStack<T> {
    pub fn new() -> Self {
        MyStack {
            stack: RefCell::new(Vec::new()),
        }
    }
    pub fn push(&self, value: T) {
        self.stack.borrow_mut().push(value);
    }
    pub fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }
}
//didn't use RefCell
// #[derive(Debug)]
// pub struct MyStack2<T> {
//     pub stack: Vec<T>,
// }
// impl<T> MyStack2<T> {
//     pub fn new() -> Self {
//         MyStack2 { stack: Vec::new() }
//     }
//     pub fn push(&mut self, value: T) {
//         self.stack.push(value);
//     }
//     pub fn pop(&mut self) -> Option<T> {
//         self.stack.pop()
//     }
// }
