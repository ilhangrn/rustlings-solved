// In this exercise, we are given a `Vec` of `u32` called `numbers` with values
// ranging from 0 to 99. We would like to use this set of numbers within 8
// different threads simultaneously. Each thread is going to get the sum of
// every eighth value with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, …
// The second thread (offset 1), will sum 1, 9, 17, …
// The third thread (offset 2), will sum 2, 10, 18, …
// …
// The eighth thread (offset 7), will sum 7, 15, 23, …
//
// Each thread should own a reference-counting pointer to the vector of
// numbers. But `Rc` isn't thread-safe. Therefore, we need to use `Arc`.
//
// Don't get distracted by how threads are spawned and joined. We will practice
// that later in the exercises about threads.

// Don't change the lines below.
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // TODO: Define `shared_numbers` by using `Arc`.
    // let shared_numbers = ???;
    let shared_numbers = Arc::new(numbers);

    // this kind of vector is used to store the join handles of the threads we spawn.
    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // TODO: Define `child_numbers` using `shared_numbers`.
        // let child_numbers = ???;
        let child_numbers = Arc::clone(&shared_numbers);
        // this clone is Arc based, so it is cheap and does not clone the underlying data.
        // similar to Rc, Arc is a reference-counting pointer, but it is thread-safe.

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        // here push is used to store the join handle of the thread we just spawned.
        join_handles.push(handle);
    }

    // here we join all the threads we spawned.
    // into_iter is used to consume them
    // not to be confused with iter which borrows them

    for handle in join_handles.into_iter() {
        // unwrap is used to panic if the thread panicked
        // without it, we would not know if the thread panicked or not

        // join is used to wait for the thread to finish
        handle.join().unwrap();
    }
}
