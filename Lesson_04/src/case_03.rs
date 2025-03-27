pub fn run() {
    let average = 3.14; // f64 stack bazlı bir primitive type.
    calculate(average); // Burada sahiplik el değiştirirken verinin kopyalanarak taşınması söz konusudur
    println!("Average value : {}", average); // Bu nedenle dönüşte average kullanılmaya devam eder
    // Dolayısıyla burada Value used after being moved [E0382] hatası söz konusu olmaz
}

fn calculate(input: f64) {
    println!("Incoming message; '{}'", input);
}
