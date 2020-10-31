pub fn run() {
    println!("Printing the message in another file!");
// Print the number 1
    println!("{}", 1);
    
// This is used to replace variables, this variable is immutable
    let text = "my text";
    println!("{}", text);
    
// This variable is muttable
    let mut text2 = "another text";
    println!("{}", text2);
    text2 = "text 2 is muttable";
    println!("{}", text2);

// Another form to replace
    println!("{name} is {years} years old", name = "Simon", years = "21"); 
}