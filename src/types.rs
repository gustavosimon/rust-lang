pub fn run() {
    let mut hello = String::from("Hello World!");
    
    println!("{}", hello);

    println!("Length: {}", hello.len());

    // Push just a char 
    hello.push('T');
    let is_hello_world = hello.contains("World");

    if is_hello_world {
        println!("yeah, it is a hello world");
    }

    // Push a string 
    hello.push_str("teste with more than 24 chars");

    // Capacity in bytes
    println!("{}", hello.capacity());

    println!("{}", hello);

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

}