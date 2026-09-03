// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`
// import random number generator
use std::{sync::Arc, sync::Mutex, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    // `Arc` provides shared ownership of a value, but it doesn't allow mutation.
    // To have a mutable shared state, you need to use `Mutex` in combination with `Arc`.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for i in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::Builder::new()
            .name(format!("worker-{i}"))
            .spawn(move || {
                let binding = thread::current();
                let thread_name = binding.name().unwrap_or("unknown").to_string();

                println!("init {}", thread_name);
                // if i odd delay negative, if even positive
                let delay = if i % 2 == 0 { 250} else { 500 };
                thread::sleep(Duration::from_millis(delay));

                // TODO: You must take an action before you update a shared value.
                println!("finished {}", thread_name);
                let mut status = status_shared.lock().unwrap();
                status.jobs_done += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        println!("Waiting next one to finish");
        handle.expect("Thread panicked").join().unwrap();
        println!("joined to main!");
    }

    // TODO: Print the value of `JobStatus.jobs_done`.
    println!("Jobs done: {}", status.lock().unwrap().jobs_done);
}
