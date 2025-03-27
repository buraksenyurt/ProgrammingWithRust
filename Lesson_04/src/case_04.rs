pub fn run() {
    /*
        run scope'u içindeki average ile calculate fonksiyonuna koplayanarak taşınan
        versiyonu arasında bir ilişki yoktur. Dolayısıyla average içeriği calculate fonksiyonuna
        alınıp değiştirilse bile run içerisindeki değeri değişmez.
    */
    let average = 3.14;
    println!("Average: {}", average);
    calculate(average);
    println!("After calculate, {}", average);

    // Gönderilen değerin değiştirilip yeni bir değer olarak geri alınması yöntemi
    let new_average = increase_one(average);
    println!("After increase, {}", new_average);
}

fn calculate(input: f64) {
    let _ = input * 0.1;
}

fn increase_one(input: f64) -> f64 {
    input + 1.0
}
