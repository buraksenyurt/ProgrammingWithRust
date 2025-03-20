use crate::data::*;
use std::env;

pub fn parse() -> Result<Command, String> {
    let _arguments: Vec<String> = env::args().collect();

    Ok(Command {
        count: 100,
        period: 1,
        metric: Metric::Both,
    })
}
