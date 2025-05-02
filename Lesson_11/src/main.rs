use std::thread;

fn main() {
    multiple_threads_sample();
    println!("After the thread calling");
}

fn calc_factorial(n: u64) -> u64 {
    (1..=n).product()
}
pub fn multiple_threads_sample() {
    let numbers = vec![10, 3, 5, 13, 8, 9, 1, 2, 17, 11, 7, 6];
    let threads_count = 4;
    let mut handles = vec![];
    let chunk_size = numbers.len() / threads_count;

    for i in 0..threads_count {
        let chunk = numbers[i * chunk_size..(i + 1) * chunk_size].to_vec();
        handles.push(thread::spawn(move || {
            let mut results = vec![];
            for num in chunk {
                println!("Thread {} processing for {}", i, num);
                results.push((num, calc_factorial(num)));
            }
            results
        }));
    }

    let mut final_results = vec![];

    for handle in handles {
        final_results.push(handle.join().unwrap());
    }

    println!("{:?}", final_results);
}
