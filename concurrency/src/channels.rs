/// mpsc stands for Multiple Producer, Single Consumer
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn channels() {
    let (tx, rx) = mpsc::channel();

    // move transmitting end into thread
    thread::spawn(move || {
        let val = String::from("hi");
        // `send` returns a Result<T, E>, if reciever is already dropped and there is
        // nowhere to send the data, then Error is returned.
        tx.send(val).unwrap();
    });

    // `recv` stands for receive. This method will block the main thread' execution and
    // wait until a value is sent down the channel.
    // `recv` returns a Result<T, E>. When the transmitter closes, `recv` will return
    // an error to signal that no more values will be coming.
    // `try_recv` does NOT block, but will return a Result<T, E> immediately.
    // Using `try_recv` is useful if this thread has other work to do while waiting for
    // messages.
    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

pub fn send_multiple_messages_and_see_receiver_waiting() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

pub fn multiple_producers_single_consumer() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}
