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
