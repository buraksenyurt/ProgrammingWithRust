use std::f64::consts::PI;

#[allow(dead_code)]
pub fn run() {
    let average = PI; // f64 stack bazlı bir primitive type.
    calculate(average); // Burada sahiplik el değiştirirken verinin kopyalanarak taşınması söz konusudur
    println!("Average value : {}", average); // Bu nedenle dönüşte average kullanılmaya devam eder
    // Dolayısıyla burada Value used after being moved [E0382] hatası söz konusu olmaz
}

#[allow(dead_code)]
fn calculate(input: f64) {
    println!("Incoming message; '{}'", input);
}
