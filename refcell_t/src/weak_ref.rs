//! Strong references are how you can share ownership of an Rc<T> instance.
//! Weak references don's express an ownership relationship, and their count doesn't
//! affect when an Rc<T> instance is cleaned up. They won't cause a reference cycle
//! b/c any cycle involving some weak references will be broken once the strong
//! reference count of values involved is 0.
//!
//! Calling Rc::downgrade gives a smart pointer of type Weak<T>. This increases the
//! weak_count. Rc<T> uses weak_count to keep track of how many Weak<T> references
//! exist. Unlike strong_count, weak_count does NOT need to be 0 for the Rc<T>
//! instance to be cleaned up.
//!
//! calling `upgrade` on Weak<T> instance -> Option<Rc<T>>
//!   - get `Some` if Rc<T> value has not been dropped.
//!   - get `None` if it has been dropped.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    /// Dropping parent will drop the child, but if a child is dropped, it has no
    /// effect on the parent due to the Weak<T> type. The child can now refer to
    /// the parent but does not own the parent.
    parent: RefCell<Weak<Node>>,
    /// We want node to own its children, and we want to share that ownership with
    /// vars so we can access each Node in the tree directly. We do this by defining
    /// Vec<T> items to be values of type Rc<Node>.
    /// We also want to modify which nodes are children of another node.
    /// To do this, we have RefCell<T> in children around the Vec<Rc<Node>>.
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn tree() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        // create a leaf node with no children
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            // Create a branch node with a leaf as one of its children.
            // We clone the Rc<Node> in `leaf` and store that in `branch`, meaning the
            // `Node` in `leaf` now has two owners: `leaf` and `branch`.
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
