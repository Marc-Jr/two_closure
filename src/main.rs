use std::cell::RefCell;
use std::rc::Rc;

fn create_counter() -> (impl FnMut(), impl Fn() -> i32) {
    let counter = Rc::new(RefCell::new(0));

    // Method 1: Increment the counter by 1
    let increment = {
        let counter = Rc::clone(&counter); 
        move || {
            *counter.borrow_mut() += 1;
        }
    };

    // Method 2: Get the current value of the counter
    let get_count = {
        let counter = Rc::clone(&counter); 
        move || {
            *counter.borrow()
        }
    };

    (increment, get_count)
}

fn main() {
    
    let (mut increment, get_count) = create_counter();


    increment();
    increment();
    increment();
    increment();

    let count = get_count();
    println!("Current count: {}", count); 
}
