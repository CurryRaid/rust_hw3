mod my_macro;
mod my_stack;
mod myrc;
use std::collections::HashMap;
fn main() {
    println!("test for hash_map! macro...");
    let map = hash_map! {
        "one" => 1,
        "two" => 2,
        "three" => 3
    };
    println!("{:?}", map);

    println!("test for my_stack...");
    let stack = my_stack::MyStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("{:?}", stack);
    println!("{:?}", stack.pop());
    println!("{:?}", stack);

    println!("test for myrc...");
    let rc = myrc::MyRc::new("abd");
    let rc2 = rc.clone();
    let rc3 = rc.clone();
    println!("rc2 and rc3 is clone from rc");
    println!("rc count: {}", rc.get_count());
    println!("rc2 count: {}", rc2.get_count());
    println!("rc3 count: {}", rc3.get_count());
    println!("the data in rc is {}", *rc);
}
