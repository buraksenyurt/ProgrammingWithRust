/*
    Scope, Ownership, Borrowing, Lifetimes
    Resource Acquisition is Intialization (RAII)
*/
mod case_00;
mod case_01;
mod case_02;
mod case_03;

fn main() {
    // case_00::run(); // value borrowed here after move hatası
    // case_01::run(); // &str kullanımı sebebiyle case 00'daki ihlal oluşmaz
    // case_02::run();
    case_03::run();
}
