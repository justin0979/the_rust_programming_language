//! `spawn` creates a new thread. When main thread completes, all threads are shut down.
//! `thread::spawn` return type is `JoinHandle<T>`, which is an owned value that, when
//! we call the `join` method on it, will wait for its thread to finish.

use std::thread;
use std::time::Duration;

pub fn end_spawn_early() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn finish_all_threads_w_join() {
    println!();
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread with join!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread with join!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

pub fn move_closures_with_threads() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
