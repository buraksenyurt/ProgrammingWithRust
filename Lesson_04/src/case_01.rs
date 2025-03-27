pub fn run() {
    let message = "Houston! We have a problem";
    do_something(message); // Burada fonksiyon veri değil de veriyi referans eden değişken taşınır
    println!("{}", message);
}

fn do_something(input: &str) {
    println!("Incoming message; '{}'", input);
}
