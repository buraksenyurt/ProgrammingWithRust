/*
    Yeni bir rust binary oluşturmak için

    cargo new hello_world
    cargo new --lib json_parser

    Çalıştırmak için

    cargo run

    Build etmek için

    cargo build
*/

// main executable/binary olarak giriş noktasıdır
fn main() {
    // ! ile biten enstrümanlar macro olarak geçer
    println!("Hello, world!");

    // 23 varsayılan 32 bit integer şeklinde saklanır (stack)
    /*
        Rust dilinde tüm değişkenler varsayılan olarak immutable (değişmez) statüsündedir.
        Açık bir şekilde değiştirilebilir olmasını istiyorsa mut keyword' ünü kullanırız.
        Aşağıdaki gibi.
    */
    let mut number = 23;
    number = 24;
    // unsigned 8 bit integer olarak 86 değerini taşıyan point isimli bir değişken(variable)
    let point: u8 = 86;

    // Noktalı sayılarda varsayılan tür 64 bitlik floating point'tir
    let pi = 3.1415;
    // Bu sefer 32 bit floating point
    let e: f32 = 2.22;

    // Boolean tür tanımı
    let is_active = true;

    // Karakter tipi örneği
    let first_letter = 'a';

    // tuple bir compound type olarak geçer
    // Farklı veri türlerinde setleri taşımak için kullanabiliriz.
    let game_parameters = (1280, 960, "Shogun Shawndown", false);
    println!("{:?}", game_parameters);
    println!("{:#?}", game_parameters);
    // Tuple elemanlarına 0,1 gibi alan adlarıyla erişebiliriz
    println!("({}x{})", game_parameters.0, game_parameters.1);

    let height = 1280;
    println!("{}", height);

    let height = 1960;
    println!("{}", height);

    let (mut width, height) = (800, 600);
    println!("İlk width değeri {}", width);
    width = 1280;
    println!("Width değiştirildi {}", width);

    let mut width_me = width;
    width_me = 1000;
    println!("Width Me değeri {}", width_me);
    println!("Width değeri {}", width);

    // varsayılan olarak değerlerin türü ne ise ona bürünen bir array
    // Diziler hep aynı veri türünden nesneleri barındırır
    // Boyutları bellidir, mutable olabilirler
    let mut points = [10, 20, 30, 40, 50];
    points[3] = 35;
    // points[99]=100; // Index out of bounds hatası verir

    // Vektörler boyutu dinamik olarak manipüle edilebilen yapılardır
    // Açık bir şekilde tür belirtilebilir (örneğin sağ taraftaki sayılar u8 olsun)
    let mut locations: Vec<u8> = vec![9, 5, 6, 1, 7, 8, 10, 20];
    println!("{:?}", locations);
    locations.push(3); // sona yeni bir eleman ekler
    println!("{:#?}", locations);

    let last_added = locations.pop();
    println!("{:?}", last_added); // pop metodu Option<T> enum döndüğünden şimdilik :? ile yazdırdık
    println!("{:#?}", locations);

    // Bir slice belirlerken bellekte ne kadar yer tutacağı bilinmez
    // Bu nedenle heap'teki bellek alanının belirli bir dilimini işaret eden bir referans ile çalışılır
    // (O nedenle & konuldu. & koymadan deneyelim)
    let _range = &locations[..=4]; // slice(dilim) ilk baştan itibaren dördündü indis dahil beş elemanı alır
    let range = &locations[3..6];
    println!("{:?}", range);

    let numbers: Vec<u16> = (50..60).collect();
    println!("{:?}", numbers);

    let mut motto = String::from("Rust ile Programlama");
    // let mut motto = "Rust ile programlama".to_string();
    // let mut motto = String::new();
    // motto.push_str("Rust");
    // motto.push_str(" ile ");
    // motto.push_str(" programlama ");

    // let motto = motto.chars().rev().collect::<String>();
    println!("{}", motto);
    motto.push_str(" !!!");
    println!("{}", motto);

    let konnichiwa = "\u{3053}\u{3093}\u{306B}\u{3061}\u{306F}";
    println!("{}", konnichiwa);
}

// Program yaşadığı sürece değeri değişmeyecek değişkenler için const kullanılabilir.
// unsigned 32 bit örnek bir sabit

// Neden constant tanımlanırken Rust veri tipini bilmek ister?
const MAX_WIDTH: u32 = 1280;
