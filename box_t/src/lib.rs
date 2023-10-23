//! Bix<T> Crate
//!
//! Using Box<T> to Point to Data on the Heap.
//! This is the most straight forward smart pointer. It allows you to store data on
//! the heap rather than the stack.
//!   - What remains on the stack is the pointer to the heap data.
//!  
//! Boxes provide only the indirection and heap allocation; they don't have any other
//! special capabilities.
//!   - indirection means that instead of storing a value directly, we should change
//!     the data structure to store the value indirectly by storing a pointer to the
//!     value instead.
//!
//! Box<T> type is a smart pointer b/c it implements the `Deref` trait, which allows
//! Box<T> values to be treated like references.
//! When a Box<T> value goes out of scope, the heap data that the box is pointing to
//! is cleaned up as well b/c of the `Drop` trait implementation.
//!
//! Most common uses:
//!   - When you have a type whose size can't be known at compile time and you want
//!     to use a value of that type in a context that requires an exact size
//!   - When you have a large amount of data and you want to transfer ownership but
//!     ensure the data won't be copied when you do so
//!   - When you want to own a value and you care only that it's a type that implements
//!     a particular trait rather than being of a specific type, aka a `trait object`
//!
//! Chapter 17 will have more use cases for boxes.

pub fn useless_box_example() {
    let b = Box::new(5); // The value of `5` is stored on the heap
    println!("b = {b}");
}

pub fn cons_list() {
    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:#?}", list);
}

/// `cons list` example in Lisp is:
///   (1, (2, (3, Nil)))
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn deref_example() {
    let x = 5;
    let y = &x; // y is a reference pointing to the value of x

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

pub fn box_deref_example() {
    let x = 5;
    let y = Box::new(x); // y is an instance of a box pointing to a copied value of x

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

use std::ops::Deref;

/// The `MyBox` type is a tuple struct with one element of type T.
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

pub fn mybox_deref_example() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

/// Deref coercion will run `Deref::deref` as many times as necessary to get a
/// reference to match the parameter's type.
/// The number of time `Deref::deref` needs to be inserted is resolved at compile time,
/// so there is no runtime penalty.
pub fn deref_coercion_ex() {
    let name = String::from("Justin");
    hello(&name);

    let m = MyBox::new(name);
    // Works bc of deref coercion. &m is a reference to a MyBox<String> value. Bc
    // MyBox<T> was implemented with the Deref trait, Rust can turn &MyBox<String>
    // into &String by calling `deref`. Rust calls `deref` again to turn &String
    // into &str.
    hello(&m);

    /*
     * WITHOUT DEREF COERCION, the above code would have to be rewritten:
     *
     *   - (*m) derefences MyBox<String> into a String.
     *   - & and [..] take a string slice of the String that is equal to the whole
     *     string.
     *
     *     1.) (*m) = String
     *     2.) &(*m) = &String
     *     3.) &(*m)[..] = &str
     */
    hello(&(*m)[..]);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: `{}`!", self.data);
    }
}

pub fn drop_example() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

pub fn drop_explicitly() {
    let c = CustomSmartPointer {
        data: String::from("some data stored in c"),
    };
    let _d = CustomSmartPointer {
        data: String::from("Some data that is in the d variable"),
    };
    println!("\nCustomerSmartPointers created.");
    drop(c);
    println!("CustomSmartPointer c dropped before the end of main");
}
