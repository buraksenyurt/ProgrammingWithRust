use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    run_mutex();
    println!("After the thread calling");
}

pub fn run_mutex() {
    let data = Arc::new(Mutex::new(0));

    let data_clone_one = Arc::clone(&data);
    let t1 = thread::spawn(move || {
        let mut num = data_clone_one.lock().unwrap();
        *num += 3;
        println!("Thread 1 has locked the data.");
        thread::sleep(Duration::from_secs(3)); // Kasıtlı olarak bekletme yapıyoruz
        println!("After 3 seconds...\nThread 1 is unlocking the data.");
    });

    let data_clone_two = Arc::clone(&data);
    let t2 = thread::spawn(move || {
        println!("Thread 2 is trying to lock the data.");
        let mut num = data_clone_two.lock().unwrap();
        *num += 5;
        println!("Thread 2 has locked and updated the data.");
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Final value: {}", *data.lock().unwrap());
}
