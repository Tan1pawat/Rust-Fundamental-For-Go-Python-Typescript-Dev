fn main() {
    // Ownership — `String` is moved, not copied
    let s1 = String::from("hello");
    let s2 = s1; // ownership moved to s2
    // println!("{}", s1); // ❌ error: s1 is no longer valid

    println!("s2 = {}", s2);

    // Borrowing — reference instead of move
    let s3 = String::from("Test");
    print_length(&s3); // borrow as immutable reference
    println!("s3 still valid: {}", s3);
}

fn print_length(s: &str) {
    println!("Length is {}", s.len());
}
