fn memory_dropped_when_outof_scope() {
    let mut s = String::from("Hello");

    // Mutable
    s.push_str(", world!");

    println!("{s}");
}


fn gives_ownership() -> String { // gives_ownership will move its return value into the function
                                 // that calls it

    let some_string = String::from("yours"); // some_string comes into scope
 
    return some_string // some_string is returned and moves out to the calling function
}


// this function takes a String and returns a String
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope

    return a_string // a_string is returned and moves out to the calling function
}


// **Borrows** a string to extract it's length
fn calculate_lenghth_without_taking_ownership(s: &String) -> usize {

    // This would panic : we cannot modify borrowed data
    // s.push_str("not working");

    return s.len()
}

// Borrows a string **and** modifies it
fn modify_a_borrowed_value(s: &mut String) {

    s.push_str("modified");
}


fn xmain() {
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
 
    let mut s2 = String::from("hello"); // s2 comes into scope

    calculate_lenghth_without_taking_ownership(&s2); // Using references prevents sending ownership
                                                     // to functions

    modify_a_borrowed_value(&mut s2); // Now this will allow modifications on the borrowed value

    println!("\n\n{s2}");

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves
                                       // its return value into s3

} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of
  // scope and is dropped.


fn ymain() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    
    let r1_size = calculate_lenghth_without_taking_ownership(&r1);
    let r2_size = calculate_lenghth_without_taking_ownership(&r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // so no problem here

    println!("{r3}");
}


// A function that takes a string of words separated by spaces and returns the first word it finds
// in that string. If the function doesn’t find a space in the string, the whole string must be one
// word, so the entire string should be returned.
fn the_slice_problem(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &c) in bytes.iter().enumerate() {
        if c == b' ' {
            return i;
        }
    }

    return s.len()
}


fn zmain() {
    let s = String::from("fuck this!");

    println!("{}", the_slice_problem(&s))
}


fn main() {
    let mut s = String::from("hello world!");

    let hello = &s[0..5];

    println!("{hello}");
}
