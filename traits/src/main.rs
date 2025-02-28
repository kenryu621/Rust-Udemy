mod drop;
mod into;
mod operator_overloading;
mod static_and_dyn_dispatch;
mod trait_parameters;
mod traits;
mod vectors_of_diff_objects;
mod why_dyn_dispatch;

fn main() {
    traits::traits();
    trait_parameters::trait_parameters();
    into::into();
    drop::drop_demo();
    operator_overloading::operator_overloading_demo();
    static_and_dyn_dispatch::static_and_dyn_dispatch_demo();
    why_dyn_dispatch::why_dyn_dispatch_demo();
    vectors_of_diff_objects::vectors_of_different_objects_demo();
}
