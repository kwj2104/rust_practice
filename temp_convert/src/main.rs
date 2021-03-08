use std::io;

fn main() {
    println!("Enter your temperature in F: ");

    let mut ftemp = String::new();

    io::stdin()
        .read_line(&mut ftemp)
        .expect("failed to read");

    let ftemp: f32 = match ftemp.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    let ctemp: f32 = (ftemp - 32.0) / 1.8;

    println!("Temperature in C: {}", ctemp);

}

