mod samples;

use procs::work_time_effort;

#[work_time_effort]
fn find_total() {
    let mut total = 0;
    for i in 0..1000 {
        total += i;
    }
    println!("Result: {}", total);
}

fn main() {
    let _total = find_total();
}
