use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_random_number(max: usize) -> usize {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Error at duration calculation");

    let nanos = duration.subsec_nanos() as usize;

    if max == 0 { 0 } else { nanos % max }
}

pub fn get_random_between(min: usize, max: usize) -> usize {
    if min > max {
        panic!("Min must be less than or equal to Max");
    }

    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Error at duration calculation");

    let nanos = duration.subsec_nanos() as usize;
    let range = max - min + 1;

    min + (nanos % range)
}

