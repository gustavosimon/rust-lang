use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Simon";
    let status = "100%";

    // println!("Command: {:?}", command);

    if command == "hello" {
        println!("Hello {}, how are you?", name);
    } else if command == "status" {
        println!("Status {}", status);
    } else {
        println!("That is a no valid command line");
    }
}
