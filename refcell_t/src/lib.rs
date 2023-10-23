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

/// A library to tracks a value against a maximum and will output a message saying how
/// close to the maximum the value is.
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent: You're at 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You're at 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Using RefCell<T> to mutate an inner value while the outer value is considered
    // immutable
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // Example of creating two mutable references in the same scope to see
            // that RefCell<T> will panic.
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
