//! Smart Pointers
//!
//! From page 334: Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>
//!  * Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have
//!    single owners.
//!  * Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows
//!    only immutable borrows checked at compile time; RefCell<T> allows immutable or
//!    mutable borrows checked at runtime.
//!  * B/c RefCell<T> allows mutable borrows checked at runtime, you can mutate the
//!    value inside the RefCell<T> even when the RefCell<T> is immutable.

use box_t;
use rc_t;
use refcell_t;

fn main() {
    box_t::useless_box_example();
    box_t::cons_list();
    box_t::deref_example();
    box_t::box_deref_example();
    box_t::mybox_deref_example();
    box_t::deref_coercion_ex();
    box_t::drop_example();
    box_t::drop_explicitly();

    rc_t::cons_list();
    rc_t::cons_list_counting();

    refcell_t::refcell_t();
}
