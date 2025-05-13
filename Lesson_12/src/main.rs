use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    multi_producer();
}

pub fn multi_producer() {
    let (transmitter, receiver) = channel();

    for i in 0..10 {
        let transmitter_clone = transmitter.clone();
        thread::spawn(move || {
            transmitter_clone
                .send(format!("Sending message is {}", i))
                .unwrap();
            thread::sleep(Duration::from_secs(2));
        });
    }

    drop(transmitter);

    for received in receiver {
        println!("Incoming message is '{}'", received);
    }

    println!("End of program");
}
