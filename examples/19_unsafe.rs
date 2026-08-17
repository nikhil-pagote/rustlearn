//! Exercise 19: unsafe Rust — the five superpowers, used sparingly.
//! `unsafe` doesn't turn off the borrow checker; it only allows five things
//! the compiler can't verify itself. Keep blocks small and wrap them in safe
//! abstractions.
//! Run: cargo run --example 19_unsafe

// Superpower 1: dereference raw pointers (*const T / *mut T).
// Raw pointers may be null, unaligned, or dangle — reading them is unsafe.
fn raw_pointer_demo() {
    let mut n = 5;
    let r1 = &n as *const i32;
    let r2 = &mut n as *mut i32;

    unsafe {
        // We (not the compiler) guarantee r1 and r2 are valid here.
        println!("raw read = {}", *r1);
        *r2 = 10;
        println!("after raw write, n = {n}");
    }
}

// Superpower 2: call an unsafe function — here, a C library function via FFI.
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn ffi_demo() {
    unsafe {
        println!("C abs(-42) = {}", abs(-42));
    }
}

// A safe abstraction over an unsafe implementation: splitting one mutable
// slice into two non-overlapping ones. The standard library's
// `split_at_mut` is written exactly this way.
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    assert!(mid <= values.len());
    let len = values.len();
    let ptr = values.as_mut_ptr();
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Superpower 3: access/modify a mutable static. This one is genuinely
// dangerous across threads — shown here single-threaded only. (Prefer
// atomics or Mutex in real code.)
static mut GREETING_COUNT: u32 = 0;

fn count_greeting() {
    unsafe {
        GREETING_COUNT += 1;
        // SAFETY argument: main is single-threaded in this program.
        let count = GREETING_COUNT;
        println!("greeting #{count}");
    }
}

// Superpowers 4 & 5 (not shown): implementing unsafe traits (like Send/Sync)
// and accessing fields of `union`s — both rare outside library code.

fn main() {
    raw_pointer_demo();
    ffi_demo();

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut v, 3);
    left[0] = 100;
    right[2] = 600;
    println!("after split mutation: {v:?}");

    count_greeting();
    count_greeting();
}
