// FFI declarations and types
mod ffi {
    use std::ffi::c_void;
    pub type handle_t = *const c_void;
    // ...
}

// A wrapper structure for some context created and maintained
// inside the C library
struct Context {
    // ...
}

// Handle is only valid as long as the Context is alive.
// Hence, I use the PhantomData marker to constrain its lifetime.
struct MyStruct<'a> {
    marker: PhantomData<&'a Context>,
    handle: ffi::handle_t,
}

impl<'a> MyStruct<'a> {
    fn new(context: &'a Context) -> Self {
        let handle: ffi::handle_t = context.new_handle();
        MyStruct {
            marker: PhantomData,
            handle
        }
    }
}

fn main() {
    // Initialize the context somewhere inside the C library
    let ctx = Context::new(unsafe {ffi::create_context()});

    // Create an instance of MyStruct
    let my_struct = MyStruct::new(&ctx);

    // ...
}