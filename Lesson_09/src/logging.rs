use crate::models::{Level, Log};
use std::io::{Write, stdout};

pub fn log() -> impl FnMut(&Log) {
    let mut error_count = 0;
    let mut warn_count = 0;
    let mut info_count = 0;
    move |l| {
        stdout()
            .write_all(format!("{}\n", l).as_bytes())
            .expect("Could not write to stdout");
        match l.level {
            Level::Error => error_count += 1,
            Level::Warn => warn_count += 1,
            Level::Info => info_count += 1,
        }
        stdout()
            .write_all(
                format!(
                    "Log Tracker: {} errors, {} warnings, {} infos\n",
                    error_count, warn_count, info_count
                )
                .as_bytes(),
            )
            .expect("Couldn't write to stdout");
    }
}
