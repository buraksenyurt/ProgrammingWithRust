use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_random_number(max: usize) -> usize {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Error at duration calculation");

    let nanos = duration.subsec_nanos() as usize;

    if max == 0 { 0 } else { nanos % max }
}
