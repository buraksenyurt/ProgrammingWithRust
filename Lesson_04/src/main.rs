/*
    Scope, Ownership, Borrowing, Lifetimes
    Resource Acquisition is Intialization (RAII)
*/
mod case_00;
mod case_01;
mod case_02;
mod case_03;
mod case_04;
mod case_05;
mod case_06;
mod case_07;
mod case_08;
mod case_09;
mod case_10;

fn main() {
    // case_00::run(); // value borrowed here after move hatası
    // case_01::run(); // &str kullanımı sebebiyle case 00'daki ihlal oluşmaz
    // case_02::run();
    // case_03::run();
    // case_04::run();
    // case_05::run();
    // case_06::run();
    // case_07::run();
    // case_08::run();
    // case_09::run();
    case_10::run();
}
