use crate::terminal::parse;
use std::thread::sleep;
use std::time::Duration;

mod terminal;

fn main() {
    if let Ok(cmd) = parse() {
        let mut system = sysinfo::System::new();

        for _ in 0..cmd.count {
            system.refresh_memory();
            println!("{} Mb", system.free_memory() / (1024 * 1024));
            sleep(Duration::from_secs(cmd.period as u64));
        }
    }
}
