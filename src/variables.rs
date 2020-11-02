pub fn run(id: i32) {
// Define a constant 
//    const ID: i32 = 001;    
    println!("{}", id);

// Multiple vars
    let ( simon_name, simon_age ) = ("Simon", "21");
    println!("{} is {} years old", simon_name, simon_age);

    let emoji = "\u{1F600}";
    println!("{}", emoji);
}