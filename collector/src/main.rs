use crate::data::Metric;
use crate::terminal::parse;
use crate::view::{print_all_metrics, print_cpu, print_memory, show_usages};
use std::thread::sleep;
use std::time::Duration;
use sysinfo::System;

mod data;
mod terminal;
mod view;

fn main() {
    match parse() {
        Ok(cmd) => {
            println!("{cmd}");

            let mut system = System::new();
            system.refresh_all();

            for _ in 0..cmd.count {
                match cmd.metric {
                    Metric::Memory => {
                        print_memory(&mut system);
                    }
                    Metric::Cpu => {
                        print_cpu(&mut system);
                    }
                    Metric::Both => {
                        print_all_metrics(&mut system);
                    }
                }
                sleep(Duration::from_secs(cmd.period as u64));
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            show_usages();
        }
    }
}
