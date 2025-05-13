use std::sync::mpsc::channel;
use std::thread;

fn main() {
    hello_channels();
}

pub fn hello_channels() {
    let (transmitter, reciever) = channel();
    let message = String::from("Sample content");

    thread::spawn(move || {
        transmitter.send(message).unwrap();
    });

    let data = reciever.recv().unwrap();
    println!("{}", data);
}
