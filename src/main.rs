use std::process;
// mod print;
mod variables;

fn main() {
    println!("My pid is {}", process::id());
//    print::run();
    variables::run();
}

