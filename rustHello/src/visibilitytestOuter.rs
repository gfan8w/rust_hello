
use visibilitytest;

///演示可见性，pub(in path), pub(crate), pub(super), and pub(self)
pub fn run(){
    visibilitytest::vis1::public_api();
    visibilitytest::vis1::submodule::my_method();
    visibilitytest::vis2::outer_mod::inner_mod::inner_mod_visible_to_anywhere_fn();

    visibilitytest::restaurant::eat_at_restaurant();
    // 因为 pub use crate::restaurant::front_of_house::hosting; 这句重新导出了，最前面加了pub，才使得下面这句话可以执行。否则，只能执行上面这句话
    visibilitytest::restaurant::hosting::add_to_waitlist();

    visibilitytest::restaurant2::eat_at_restaurant();

}


