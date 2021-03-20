use std::fs::File;
use std::io::{Read, Result};

pub fn run() {
    match read_to_string() {
        Ok(_) => (),
        Err(_) => println!("Deu ruim"),
    }
}

fn read_to_string() -> Result<()> {
    let mut file = File::open("./resources/file.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    println!("{}", data);
    Ok(())
}
