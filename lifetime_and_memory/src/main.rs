mod arc_var_and_mutex;
mod borrowing;
mod lifetime;
mod lifetime_in_struct;
mod ownership;
mod ref_counted_var;

fn main() {
    ownership::ownership_demo();
    borrowing::borrowing_demo();
    lifetime::lifetime_demo();
    lifetime_in_struct::lifetime_in_struct_demo();
    ref_counted_var::ref_counted_var_demo();
    arc_var_and_mutex::arc_var_and_mutex_demo();
}
