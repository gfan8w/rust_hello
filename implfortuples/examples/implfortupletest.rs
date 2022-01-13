use impl_trait_for_tuples::impl_for_tuples;

#[impl_for_tuples(5)]
trait Notify {
    fn notify(&self);
}

fn main() {
    println!("Hello, world!");
}