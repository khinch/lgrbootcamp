/*
 * https://doc.rust-lang.org/nightly/edition-guide/rust-2024/static-mut-references.html
 */

static mut COUNTER: u32 = 0;

fn increment() {
    unsafe {
        COUNTER += 1;
    }
}

#[allow(static_mut_refs)]
pub fn demo_of_mutable_static_variables() {
    for _ in 0..10 {
        increment();
    }

    unsafe {
        println!("Counter: {:?}", COUNTER);
    }
}
