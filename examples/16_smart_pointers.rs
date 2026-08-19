//! Exercise 16: smart pointers — Box, Rc, RefCell.
//! Smart pointers own data and add capabilities beyond plain references:
//! heap allocation (Box), shared ownership (Rc), interior mutability (RefCell).
//! Run: cargo run --example 16_smart_pointers

use std::cell::RefCell;
use std::rc::Rc;

// Box<T>: puts data on the heap. Enables recursive types — the compiler
// needs a known size, and `Box<List>` is pointer-sized.
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn list_sum(list: &List) -> i32 {
    match list {
        Cons(value, next) => value + list_sum(next),
        Nil => 0,
    }
}

fn main() {
    // Box: heap allocation, single owner.
    let boxed = Box::new(5);
    println!("boxed = {boxed}");

    // A recursive list: 1 -> 2 -> 3 -> Nil
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list = {list:?}");
    println!("list sum = {}", list_sum(&list));

    // Rc<T>: reference counting — multiple owners of the same data
    // (single-threaded only; threads need Arc, see exercise 17).
    let shared = Rc::new(String::from("shared data"));
    println!("count after creation = {}", Rc::strong_count(&shared));
    let owner_a = Rc::clone(&shared);
    let owner_b = Rc::clone(&shared);
    println!("count after two clones = {}", Rc::strong_count(&shared));
    println!("owner_a = {owner_a}, owner_b = {owner_b}");
    drop(owner_a);
    println!(
        "count after dropping owner_a = {}",
        Rc::strong_count(&shared)
    );

    // RefCell<T>: borrow rules checked at *runtime* instead of compile time.
    // Combined with Rc, this gives shared + mutable data (single-threaded).
    let log = Rc::new(RefCell::new(vec![String::from("start")]));
    let log_writer = Rc::clone(&log);
    log_writer.borrow_mut().push(String::from("event"));
    log_writer.borrow_mut().push(String::from("done"));
    println!("log = {:?}", log.borrow());
}
