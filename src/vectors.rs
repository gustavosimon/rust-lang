pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 10, 20, 30, 40];

    // add a value in the end of the vector
    numbers.push(5);
    numbers.push(27);
    // pop off the last value
    numbers.pop();

    println!("{:?}", numbers);
    // get a single value in a vector
    println!("{}", numbers[2]);
    // get the vector length
    println!("{}", numbers.len());
    // Stack memory allocated to the vector
    println!("Stack: {} bytes", std::mem::size_of_val(&numbers));

    // Looping the vector
    for number in numbers.iter() {
        println!("\n{}", number);
    }
}
