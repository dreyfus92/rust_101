// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you
// need to modify or own string data


pub fn run(){
    let hello = "Hello";
    let mut hello2 = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());

    // Push char
    hello2.push('W');

    // Push string
    hello2.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello2.capacity());

    // Check if empty
    println!("Is Empty: {}", hello2.is_empty());

    // Contains
    println!("Contains 'World' {}", hello2.contains("World"));

    // Replace
    println!("Replace: {}", hello2.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello2.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}