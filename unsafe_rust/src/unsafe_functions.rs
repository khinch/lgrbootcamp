struct Foo {}

impl Foo {
    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn bar() {
        let mut s = "raw string".to_string();
        let raw = &mut s as *mut String;
        (*raw).push_str("!");
        println!("{}", *raw);
    }
}

#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn baz() {
    Foo::bar();
}

pub fn demo_of_unsafe_functions() {
    // Functions and methods marked as unsafe can only be called from unsafe blocks
    unsafe {
        Foo::bar();
        baz();
    }
}
