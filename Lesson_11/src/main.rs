use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    poisoning_case_logging();
    println!("Everything is good!");
}
pub fn poisoning_case_logging() {
    let log_file = Arc::new(Mutex::new(
        File::create("system.log").expect("Unable to create log file"),
    ));
    let log_file_clone = Arc::clone(&log_file);

    let handle = thread::spawn(move || {
        let mut file = log_file_clone.lock().unwrap();
        writeln!(file, "Thread 1: Writing the system health status").unwrap();
        panic!("Errors occurred while writing to the log file!");
    });

    let log_file_clone = Arc::clone(&log_file);
    let handle_2 = thread::spawn(move || {
        let mut file = log_file_clone.lock().unwrap();
        thread::sleep(std::time::Duration::from_secs(3));
        writeln!(file, "Thread 2: Attempting to write").unwrap();
    });

    let log_file_clone = Arc::clone(&log_file);
    let recovery_handle = thread::spawn(move || {
        let mut file = log_file_clone
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        thread::sleep(std::time::Duration::from_secs(3));
        writeln!(file, "Thread 2: Recovering from poisoned state").unwrap();
    });

    let _ = handle.join();
    let _ = handle_2.join();
    let _ = recovery_handle.join();

    println!("Log file operations completed");
}
