//! Rc<T>, the Reference Counted Smart Pointer
//!
//! We use Rc<T> type when we want to allocate some data on the heap for multiple
//! parts of our program to read and we can't determine at compile time which part will
//! finish using the data last.
//! Rc<T> is only for use in single-threaded scenarios.

use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn cons_list() {
    use List::{Cons, Nil};

    /*
     * Rc::clone is used rather than `a.clone()` since `a.clone()` makes a deep copy.
     * Rc::clone only increments the reference count to `a`.
     * Rc::clone will not give performance problems like `clone`.
     */
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

pub fn cons_list_counting() {
    use List::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("\nCount after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("Count after creating c = {}", Rc::strong_count(&a));
    }
    println!("Count after c goes out of scope = {}", Rc::strong_count(&a));
}
