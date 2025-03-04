use std::cell::RefCell;
use std::rc::Rc;

// Function that returns an object with two methods (closures)
fn create_counter() -> (impl FnMut(), impl Fn() -> i32) {
    let counter = Rc::new(RefCell::new(0)); // Use Rc<RefCell> to share ownership

    // Method 1: Increment the counter by 1
    let increment = {
        let counter = Rc::clone(&counter); // Clone Rc to share ownership
        move || {
            *counter.borrow_mut() += 1;
        }
    };

    // Method 2: Get the current value of the counter
    let get_count = {
        let counter = Rc::clone(&counter); // Clone Rc to share ownership
        move || {
            *counter.borrow()
        }
    };

    (increment, get_count)
}

fn main() {
    // Create the counter object
    let (mut increment, get_count) = create_counter();

    // Increment the counter
    increment();
    increment();
    increment();
    increment();

    // Get the current value of the counter
    let count = get_count();
    println!("Current count: {}", count); // Output: Current count: 3
}
