#[allow(dead_code)]
pub fn run() {
    // message isimli değişken String türünden. (Heap üzerinde allocation söz konusu)
    let message = String::from("Houston! We have a problem");
    do_something(message); // Burada message değişkenin sahipliği (ownership) do_something metoduna geçer
    // Uyarı! Hatayı görmek için aşağıdaki satırı etkinleştirelim
    // println!("{}", message); // do_something içerisinde drop edilmiş olan bellek bölgesini message işaret etmeye devam edebilir
    // message değişkenin sahipliği do_something'e geçtiği için  value borrowed here after move hatası alınır
}

#[allow(dead_code)]
fn do_something(input: String) {
    println!("Incoming message; '{}'", input);
    // Bir değişkenin ömrü tanımlandığı scope sonlanınca biter(default)
    // do_something fonksiyonundan çıkılırken input değişkeni bellekten düşürülür(drop)
}
