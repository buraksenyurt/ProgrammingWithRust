use crate::data::Metric;
use crate::terminal::parse;
use std::thread::sleep;
use std::time::Duration;
use sysinfo::System;

mod data;
mod terminal;

fn main() {
    if let Ok(cmd) = parse() {
        let mut system = sysinfo::System::new();
        system.refresh_all();

        for _ in 0..cmd.count {
            match cmd.metric {
                Metric::Memory => {
                    system.refresh_memory();
                    println!("{} Mb", system.free_memory() / (1024 * 1024));
                }
                Metric::Cpu => {
                    system.refresh_cpu_usage();
                    print_metrics(&mut system);
                }
                Metric::Both => {
                    system.refresh_cpu_usage();
                    println!("{} Mb", system.free_memory() / (1024 * 1024));
                    print_metrics(&mut system);
                }
            }
            sleep(Duration::from_secs(cmd.period as u64));
        }
    }
}

fn print_metrics(system: &mut System) {
    for (id, cpu) in system.cpus().iter().enumerate() {
        print!("{} ({:2.2} %)", id, cpu.cpu_usage());
    }
    println!();
}
