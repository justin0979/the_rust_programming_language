/// mpsc stands for Multiple Producer, Single Consumer
use std::sync::mpsc;
use std::thread;

pub fn channels() {
    let (tx, rx) = mpsc::channel();

    // move transmitting end into thread
    thread::spawn(move || {
        let val = String::from("hi");
        // `send` returns a Result<T, E>, if reciever is already dropped and there is
        // nowhere to send the data, then Error is returned.
        tx.send(val).unwrap();
    });

    // Receiving the value "hi".
    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
