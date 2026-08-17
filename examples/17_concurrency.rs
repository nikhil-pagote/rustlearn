//! Exercise 17: fearless concurrency — threads, channels, Arc + Mutex.
//! The ownership rules from exercise 4 are what make shared-state threading
//! safe: the compiler rejects data races before the program can run.
//! Run: cargo run --example 17_concurrency

use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn main() {
    // `thread::spawn` starts a thread; the closure must be `move` so the
    // thread owns (not borrows) what it uses — no dangling references.
    let handle = thread::spawn(move || {
        let squares: Vec<i32> = (1..=5).map(|n| n * n).collect();
        squares
    });
    // `join` blocks until the thread finishes and hands back its return value.
    let squares = handle.join().unwrap();
    println!("squares from worker thread = {squares:?}");

    // Message passing: mpsc = multi-producer, single-consumer channel.
    // Threads share by *communicating* instead of sharing memory.
    let (tx, rx) = mpsc::channel();
    let mut senders = Vec::new();
    for id in 0..3 {
        let tx = tx.clone(); // each producer gets its own sending end
        senders.push(thread::spawn(move || {
            tx.send(format!("hello from worker {id}")).unwrap();
        }));
    }
    drop(tx); // drop the original sender so the loop below can end
    for received in &rx {
        println!("channel received: {received}");
    }
    for s in senders {
        s.join().unwrap();
    }

    // Shared state: Arc (atomic Rc for threads) + Mutex (mutual exclusion).
    // Arc gives multiple owners; Mutex ensures one thread mutates at a time.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                // `lock` blocks until this thread holds the mutex; the guard
                // unlocks automatically when it goes out of scope.
                let mut n = counter.lock().unwrap();
                *n += 1;
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("counter after 4 threads x 100 increments = {}", counter.lock().unwrap());
}
