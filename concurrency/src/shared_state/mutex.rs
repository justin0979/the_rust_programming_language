use std::sync::{Arc, Mutex};
use std::thread;

pub fn single_threaded_mutex_example() {
    let m = Mutex::new(5);

    {
        // `lock` will block the current thread so it channel do any work until it's our
        // turn to have the lock.
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

pub fn multi_thread_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        // reminder: `join` lets all the threads finish
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
