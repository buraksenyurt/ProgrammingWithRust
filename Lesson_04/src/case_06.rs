#[allow(dead_code)]
pub fn run() {
    let mut message = "Houston".to_string();
    println!("{}", message);
    change(&mut message);
    println!("{}", message);
}

#[allow(dead_code)]
fn change(input: &mut String) {
    input.push_str("! We have a problem");
}
