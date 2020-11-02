pub fn run() {
    let person: (&str, &str, i8) = ("Simon", "Novo Hamburgo", 21);

    println!("{} is from {} and {}", person.0, person.1, person.2);
}