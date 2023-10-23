//! RefCell<T> and the Interior Mutability Pattern
//!
//! Interior mutability is a design pattern in Rust that allows you to mutate data even
//! when there are immutable references to that data; normally this action is not
//! allowed due to the borrowing rules.

pub fn refcell_t() {
    println!("\nRefCell<T>\n");
}
