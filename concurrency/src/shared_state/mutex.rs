use std::sync::Mutex;

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
