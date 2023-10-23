//! RefCell<T> and the Interior Mutability Pattern
//!
//! Interior mutability is a design pattern in Rust that allows you to mutate data even
//! when there are immutable references to that data; normally this action is not
//! allowed due to the borrowing rules.
//!
//! Unlike Rc<T>, the RefCell<T> type represents single ownership over the data it
//! holds.
//! Difference b/t RefCell<T> and Box<T>? Recall the borrowing rules from ch 4:
//!   - At any given time, you can have EITHER one mutable reference or any number
//!     immutable references (but not both).
//!   - References must always be valid.
//!
//! With references and Box<T>, the borrowing rules' invariants are enforced at compile
//! time. With references, if you break these rules, you'll get a compiler error.
//! With RefCell<T>, these invariants are enforced at runtime. With RefCell<T>, if you
//! break these rules, your program will panic and exit.

pub fn refcell_t() {
    println!("\nRefCell<T>\n");
}
