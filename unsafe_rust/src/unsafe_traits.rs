trait Foo {
    unsafe fn bar(&self);
}

#[allow(unsafe_op_in_unsafe_fn)]
impl Foo for String {
    unsafe fn bar(&self) {
        let mut s = "unsafe bar".to_string();
        let raw = &mut s as *mut String;
        (*raw).push_str("!");
        println!("{}", *raw);
    }
}

pub fn demo_of_unsafe_traits() {
    let baz = "baz".to_string();
    unsafe {
        baz.bar();
    }
}
