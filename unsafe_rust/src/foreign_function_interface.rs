#[link(name = "adder", kind = "static")]
unsafe extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

pub fn ffi_c() {
    let x: i32;
    unsafe {
        x = add(1, 2);
    }
    println!("x from FFI is: {x}");
}
