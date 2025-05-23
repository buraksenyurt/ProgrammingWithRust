use std::f64::consts::PI;

#[allow(dead_code)]
pub fn run() {
    let mut average = PI; // 3.14;
    println!("Average: {}", average);
    decrease_one(&mut average);
    println!("After decrease, {}", average);
}

// decrease_one fonksiyonu input verisini referans yoluyla alır
#[allow(dead_code, clippy::let_unit_value)]
fn decrease_one(input: &mut f64) {
    // * operatörü dereference anlamındadır (deref)
    // yani & ile işaret ettiğim referans değişkenin asıl değerini almak için * ile ilerleriz
    // *input -= 1.0;
    let _ = *input -= 1.0;
}
