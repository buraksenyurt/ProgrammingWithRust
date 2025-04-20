use std::io::{self, BufRead, Write};

fn write_to_file() -> io::Result<()> {
    let mut input = String::new();
    println!("Please enter some text:");

    io::stdin().read_line(&mut input)?;
    println!("Your text is: {}", input.trim());

    Ok(())
}

fn sum() -> io::Result<i32> {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Please enter the first number:");
    io::stdin().read_line(&mut input1)?;

    println!("Second number:");
    io::stdin().read_line(&mut input2)?;

    let x: i32 = input1.trim().parse().expect("Please enter a number!");
    let y: i32 = input2.trim().parse().expect("Please enter a number!");

    Ok(x + y)
}

fn read_loop() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    println!("Please enter some text (Ctrl+Z for exit):");
    for line in reader.lines() {
        let line = line?;
        println!("Input: {}", line);
    }

    Ok(())
}

/*
    Çalıştırmak için;

    cat logs.dat | cargo run
*/
fn read_from_pipe() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    println!("Data is retrieving...");
    for line in reader.lines() {
        let line = line?;
        println!("Data: {}", line);
    }

    Ok(())
}

/*
    Çalıştırmak için;

    cargo run > logs.txt
*/
fn write_log() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    writeln!(handle, "This will be written to a file.")?;

    Ok(())
}

/*
    Çalıştırmak için;

    cat logs.dat | cargo run > output_logs.txt
*/
fn double_usage() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let reader = stdin.lock();
    let mut writer = stdout.lock();

    for line in reader.lines() {
        let line = line?;
        writeln!(writer, "Data received {}", line)?;
    }
    Ok(())
}

pub fn run() -> io::Result<()> {
    // write_to_file()?;
    //
    // let total = sum()?;
    // println!("Total: {}", total);
    //
    // read_loop()?;

    // read_from_pipe()?;

    // write_log()?;

    double_usage()?;

    Ok(())
}
