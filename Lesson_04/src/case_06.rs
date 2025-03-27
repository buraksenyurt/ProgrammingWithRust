pub fn run() {
    let mut message = "Houston".to_string();
    println!("{}", message);
    change(&mut message);
    println!("{}", message);
}

fn change(input: &mut String) {
    input.push_str("! We have a problem");
}
