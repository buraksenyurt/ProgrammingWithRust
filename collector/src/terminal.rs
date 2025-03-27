use crate::data::*;
use std::env;

pub fn parse() -> Result<Command, String> {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // early return
    if args.len() < 2 {
        return Err("Wrong number of arguments".to_string());
    }

    if args[1].to_string() != "kind" {
        return Err("Invalid kind".to_string());
    }
    if args[3] != "count" {
        return Err("Invalid count".to_string());
    }
    if args[5] != "period" {
        return Err("Invalid period".to_string());
    }

    Ok(Command {
        count: args[4].parse::<u32>().unwrap_or(20),
        period: args[6].parse::<u8>().unwrap_or(1),
        metric: match args[2].to_lowercase().as_str() {
            "cpu" => Metric::Cpu,
            "mem" => Metric::Memory,
            _ => Metric::Both,
        },
    })
}
