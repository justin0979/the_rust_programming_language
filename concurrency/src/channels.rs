/// mpsc stands for Multiple Producer, Single Consumer
use std::sync::mpsc;
use std::thread;

pub fn channels() {
    let (tx, rx) = mpsc::channel();

    // move transmitting end into thread
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    })
}
