pub fn run() {
    let numbers: [i32; 5] = [1, 10, 20, 30, 40];
    println!("{:?}", numbers);
    // get a single value in a array
    println!("{}", numbers[2]);
    // get the array length
    println!("{}", numbers.len());
    // Stack memory allocated to the array
    println!("Stack: {} bytes", std::mem::size_of_val(&numbers));
}
