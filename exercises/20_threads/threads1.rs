// This program spawns multiple threads that each runs for at least 250ms, and
// each thread returns how much time it took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Thread {i} done");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        // TODO: Collect the results of all threads into the `results` vector.
        // Use the `JoinHandle` struct which is returned by `thread::spawn`.
        // here join means to wait for the thread to finish and get its return value.
        // but why we don't use word wait:
        // 1. the term join comes from CS terminology for fork-join used in parallel programming.
        // there is main thread and we forked 10 threads,
        // now we want to join them back to main thread. 
        // It seems like wait but we join them back to main thread.
        // 2. wait is overloader, used for lots of things like:
        let result = handle.join().unwrap();
        results.push(result);
    }

    if results.len() != 10 {
        panic!("Oh no! Some thread isn't done yet!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Thread {i} took {result}ms");
    }
}
