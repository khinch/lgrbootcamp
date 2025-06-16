pub fn demo_of_dereferencing() {
    declaring_raw_pointers();
    fibonacci(10); // Overflows at 94
    multiple_references();
}

fn declaring_raw_pointers() {
    let mut s = "This is a string".to_string();

    // Creating raw pointers is safe
    let raw1 = &s as *const String; // immutable raw pointer
    let raw2 = &mut s as *mut String; // mutable raw pointer
    let address = 0x012345usize;
    let raw3 = address as *const String; // We can be fairly certain this is invalid

    println!("{}", s);

    // Dereferencing raw pointers is unsafe
    unsafe {
        (*raw2).push_str("!");
        println!("{}", *raw1);

        // println!("{}", *raw3); // Segmentation fault (core dumped)
        println!(
            "Memory locations...\nraw1: {:p}\nraw2: {:p}\nraw3: {:p}",
            raw1, raw2, raw3
        );
    }

    s.push_str("!");
    println!("{}", s);
}

fn fibonacci(limit: u8) {
    let (mut a, mut b) = (1u64, 0u64);
    let mut c: u64 = 0;
    let ptr_a = &mut a as *mut u64;
    let ptr_b = &mut b as *mut u64;
    let ptr_c = &mut c as *mut u64;
    for _ in 0..limit {
        unsafe {
            *ptr_c = *ptr_a + *ptr_b;
            println!("{}", *ptr_c);
            *ptr_a = *ptr_b;
            *ptr_b = *ptr_c;
        }
    }
}

macro_rules! ptr {
    ($type:ty, $var:ident) => {
        &mut $var as *mut $type
    };
}

fn multiple_references() {
    let mut x = 20;
    let ptr1 = ptr!(i32, x);
    let ptr2 = ptr!(i32, x);
    println!("x: {x}");

    unsafe {
        *ptr1 = *ptr1 * 2;
        *ptr2 = *ptr2 * 2;
        *ptr2 = *ptr1 * 2;
    }
    println!("x * 8 = {x}");
}
