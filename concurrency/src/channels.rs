/// mpsc stands for Multiple Producer, Single Consumer
use std::sync::mpsc;

pub fn channels() {
    let (tx, rx) = mpsc::channel();
}
