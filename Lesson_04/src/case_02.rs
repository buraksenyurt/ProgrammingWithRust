#[allow(dead_code)]
pub fn run() {
    let message = String::from("Houston! We have a problem");
    do_something(&message); // & sebebiyle verinin kendisini değil referansını gönderiyoruz
    println!("{}", message);
}

#[allow(dead_code)]
fn do_something(input: &String) {
    println!("Incoming message; '{}'", input);
} // scope sonu, sadece kopylanan referans düşer, run fonksiyonundaki orjinal veri yaşamaya devam eder
