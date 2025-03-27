pub fn run() {
    let message = "Houston";
    println!("{}", message);
    let new_message = change(&message);
    println!("{}", new_message);
}

fn change(input: &str) -> String {
    let mut output = String::from(input);
    output.push_str("!We have a problem");
    output
}
