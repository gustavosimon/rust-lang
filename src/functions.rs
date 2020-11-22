pub fn run() {
    greetings("Hello", "Simon");
    println!("{}", add(5, 7));

    // Closure function
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("{}", add_nums(7, 14));
}

fn greetings(greet: &str, name: &str) {
    println!("{}, {}", greet, name);
}

// The arrow sintax is the return of the function
fn add(n1: i32, n2: i32) -> i32 {
    // return n1 + n2;
    // can return the value with missing the semicolumn
    n1 + n2
}
